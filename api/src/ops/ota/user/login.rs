use self::super::super::super::constraints::{SCrypt64Length, StringLength, HexString, NonEmpty, EMail};
use self::super::super::super::super::util::{SCRYPT_IDEAL_PARAMS, mul_str};
use self::super::super::super::{tables, GenericError, LoginError, User};
use diesel::{ExpressionMethods, FilterDsl, FirstDsl};
use crypto::scrypt::{scrypt_simple, scrypt_check};
use rocket::request::{FormItems, FromForm};
use diesel::sqlite::SqliteConnection;
use rocket::request::FromFormValue;
use std::marker::PhantomData;
use rocket::http::RawStr;
use std::borrow::Cow;
use serde_json;
use base64;


/// Log-in *and* user-create form – refer to `doc/user.md`.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # extern crate diesel;
/// # #[macro_use]
/// # extern crate rocket;
/// # use std::fs;
/// # use std::env::temp_dir;
/// # use rocket::request::Form;
/// # use rocket::response::content::Json;
/// # use diesel::sqlite::SqliteConnection;
/// # use sudoku_backend::ops::setup::DatabaseConnection;
/// # use sudoku_backend::ops::{GenericError, LoginError, LoginForm};
/// # fn work(_: &SqliteConnection, _: i32, _: bool) -> Result<String, GenericError> {
/// #     Ok("henlo".to_string())
/// # }
/// #[post("/login", data="<form>")]
/// fn login(db: DatabaseConnection, form: Form<LoginForm>)
///     -> Result<String, Result<Json<GenericError>, Json<LoginError>>>
/// {
///     let (user_id, is_admin) = form.into_inner().validate(&db).map_err(|e| Err(Json(e)))?;
///     work(&db, user_id, is_admin).map_err(|e| Ok(Json(e)))
/// }
///
/// fn main() {
/// #   let database_file =
/// #     ("$ROOT/sudoku-backend.db".to_string(),
/// #      temp_dir().join("sudoku-backend-doctest").join("ops-ota-user-LoginForm").join("sudoku-backend.db"));
/// #   fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
/// #   /*
///     let database_file: (String, PathBuf) = /* obtained elsewhere */;
/// #   */
///     rocket::ignite()
///         .manage(DatabaseConnection::initialise(&database_file))
///         .mount("/", routes![login])
///         .launch();
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginForm {
    /// Username, non-empty.
    pub login: Result<NonEmpty, ()>,

    /// Scrypted user password, constant width.
    pub password: Result<HexString<SCrypt64Length>, Option<Option<usize>>>,

    /// User's e-mail. Need exist for user creation.
    pub email: Option<Result<EMail, Option<&'static str>>>,

    /// Error parsing the box, see `doc/user.md`.
    pub ext_error: Result<(), (Cow<'static, str>, bool)>,
}

#[derive(Deserialize)]
struct LoginFormData {
    login: String,
    password: String,
    email: Option<String>,
}

impl LoginForm {
    /// Get a list of errors.
    ///
    /// Only applicable if password couldn't be parsed.
    pub fn errors(&self) -> Vec<GenericError> {
        self.password
            .as_ref()
            .err()
            .map(|e| GenericError { reason: match e {
                Some(Some(s)) => format!("derived password of invalid length {}, please report to administrator", s).into(), // FlashMessageClass::Danger
                Some(None) => "Derived password contained non-hexadecimal character, please contact administrator".into(), // FlashMessageClass::Danger
                None => "couldn't parse password; try again".into(), // FlashMessageClass::Warning
            }})
            .into_iter()
            .chain(self.ext_error.as_ref().err().map(|&(ref s, _e)| {
                GenericError { reason: s.clone() } // if e { FlashMessageClass::Danger } else { FlashMessageClass::Warning }
            }))
            .collect()
    }

    /// Attempt to log the user in with the stored login and password.
    ///
    /// Returns user's `(id, is_admin)` on success.
    pub fn validate(&self, db: &SqliteConnection) -> Result<(i32, bool), LoginError> {
        match (self.login.as_ref(), self.password.as_ref()) {
            (Ok(&NonEmpty(ref login)), Ok(&HexString(ref password, _))) => {
                let user: User = tables::users::table.filter(tables::users::username.eq(login))
                    .first(db)
                    .map_err(|_| ())?;

                if scrypt_check(&password.to_lowercase(), &user.password) == Ok(true) {
                    Ok((user.id.unwrap(), user.is_admin))
                } else {
                    Err(())?
                }
            }
            _ => Err(())?,
        }
    }

    /// Attempt the conversion to a [`User`](struct.User.html).
    ///
    /// Returns `self`, optionally with an additional error, on error.
    pub fn into_user(mut self) -> Result<User, LoginForm> {
        if self.login.is_err() || self.password.is_err() || (self.email.is_some() && self.email.as_ref().unwrap().is_err()) || self.ext_error.is_err() {
            return Err(self);
        }

        match scrypt_simple(&self.password.as_ref().unwrap().0.to_lowercase(), &SCRYPT_IDEAL_PARAMS) {
            // TODO: email
            Ok(scrypted) => {
                Ok(User::new(self.login.unwrap().0,
                             scrypted,
                             self.email.into_iter().flat_map(|e| e).next().map(|e| e.0).unwrap_or_else(|| "not@an.email".to_string())))
            }
            Err(err) => {
                self.ext_error = Err((format!("derivation error: {}", err).into(), true));
                Err(self)
            }
        }
    }
}

impl From<User> for LoginForm {
    fn from(user: User) -> LoginForm {
        LoginForm {
            login: Ok(NonEmpty(user.username)),
            password: Ok(HexString(mul_str("0", SCrypt64Length::LENGTH), PhantomData)),
            email: Some(Ok(EMail(user.email))),
            ext_error: Ok(()),
        }
    }
}

impl<'f> FromForm<'f> for LoginForm {
    type Error = ();

    fn from_form(it: &mut FormItems<'f>, strict: bool) -> Result<Self, Self::Error> {
        fn err_login<C: Into<Cow<'static, str>>>(f: C, err: bool) -> LoginForm {
            LoginForm {
                login: Err(()),
                password: Err(None),
                email: None,
                ext_error: Err((f.into(), err)),
            }
        }


        for (k, v) in it {
            if k == "data" {
                let v = if let Ok(v) = base64::decode_config(v.percent_decode_lossy().as_ref(), base64::STANDARD) {
                    if let Ok(v) = String::from_utf8(v) {
                        v
                    } else {
                        return Ok(err_login("login not valid UTF-8; try again", false));
                    }
                } else {
                    return Ok(err_login("couldn't decode login data, please contact administrator", true));
                };

                if let Ok((l, p, e)) = serde_json::from_str::<LoginFormData>(&v).map(|d| {
                    (NonEmpty::from_form_value(RawStr::from_str(&d.login)),
                     HexString::<SCrypt64Length>::from_form_value(RawStr::from_str(&d.password)),
                     d.email.map(|e| EMail::from_form_value(RawStr::from_str(&e))))
                }) {
                    return Ok(LoginForm {
                        login: l,
                        password: p,
                        email: e,
                        ext_error: Ok(()),
                    });
                } else {
                    return Ok(err_login("couldn't parse login data, please contact administrator", true));
                }
            } else {
                if strict {
                    return Ok(err_login("extraneous login data; try again", false));
                }
            }
        }

        Ok(err_login("missing login data in login form; try again", false))
    }
}
