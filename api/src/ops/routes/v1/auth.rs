//! Auth-related routes.
//!
//! To be mounted on "/api/v1/auth".
//!
//! Refer to [doc/user.md](../../../doc/user/) for the specifics.


use self::super::super::super::errors::{GenericErrorSeverity, GenericError, LoginError};
use self::super::super::super::{SanitisedUserData, LoginForm, Session, User};
use self::super::super::super::setup::DatabaseConnection;
use rocket::response::status::Custom;
use rocket::http::{Cookies, Status};
use rocket::request::Form;
use rocket_contrib::Json;
use std::cmp;


/// Log the user in.
///
/// Check the received form.
/// If login exists and password verifies correct and not a dupe,
/// update session to reflect permissions.
#[post("/login", data="<form>")]
pub fn login(db: DatabaseConnection, mut cookies: Cookies, form: Form<LoginForm>)
             -> Result<Result<Custom<Json<SanitisedUserData>>, Custom<Json<LoginError>>>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    let form = form.into_inner();

    if form.email.is_some() {
        return Err(Custom(Status::BadRequest,
                          Json(("email field provided in login form", GenericErrorSeverity::Warning).into())));
    }

    if let Ok((user_id, is_admin)) = form.validate(&db) {
        if session.log_in(user_id, is_admin, &db).is_ok() {
            if let Ok(user) = User::get_by_id(user_id, &db) {
                return Ok(Ok(Custom(Status::Accepted, Json(user.into()))));
            } else {
                // This should be a 500, but that would leak "those are correct credentials", so we're falling through to the default 401
            }
        } else {
            // Likewise ^^
        }
    }

    Ok(Err(Custom(Status::Unauthorized, Json(LoginError {}))))
}

/// Log the user out, if it is logged in.
#[post("/logout")]
pub fn logout(db: DatabaseConnection, mut cookies: Cookies) -> Result<Custom<&'static str>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    if session.user_id.is_some() {
        if session.log_out(&db).is_err() {
            // This should be a 500, but that would leak "the user is logged in", so we're falling through to the default 204
        }
    }

    Ok(Custom(Status::NoContent, ""))
}

/// Create a new account.
///
/// Check the received form.
/// If login doesn't exist, create new user.
#[post("/new", data="<form>")]
pub fn create_account(db: DatabaseConnection, mut cookies: Cookies, form: Form<LoginForm>)
                      -> Result<Custom<Json<SanitisedUserData>>, Custom<Json<GenericError>>> {
    let _session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    let form = form.into_inner();

    if form.email.is_none() {
        return Err(Custom(Status::BadRequest,
                          Json(("email field missing in login form", GenericErrorSeverity::Warning).into())));
    }

    match form.into_user() {
        Ok(user) => {
            if user.add(&db).is_ok() {
                Ok(Custom(Status::Created, Json(user.into())))
            } else {
                Err(Custom(Status::Conflict,
                           Json(("failed to add user or user exists", GenericErrorSeverity::Warning).into())))
            }
        }
        Err(form) => {
            Err(Custom(Status::BadRequest,
                       Json(form.errors()
                           .into_iter()
                           .fold((true,
                                  GenericError {
                                      reason: "".into(),
                                      severity: GenericErrorSeverity::Lowest,
                                  }),
                                 |(first, GenericError { reason, severity }), GenericError { reason: new_reason, severity: new_severity }| {
                    (false,
                     GenericError {
                         reason: reason + if first { "" } else { "; " } + new_reason,
                         severity: cmp::max(severity, new_severity),
                     })
                })
                           .1)))
        }
    }
}

/// Get data for the currently logged-in user
#[get("/user_data")]
pub fn user_data(db: DatabaseConnection, mut cookies: Cookies) -> Result<Json<SanitisedUserData>, Custom<Json<GenericError>>> {
    let session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    if let Some(uid) = session.user_id {
        Ok(Json(User::get_by_id(uid, &db).map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?.into()))
    } else {
        Err(Custom(Status::Unauthorized, Json(("User not logged in", GenericErrorSeverity::Warning).into())))
    }
}
