//! Various verbose weak type-system-level constraints.


use rocket::request::FromFormValue;
use std::marker::PhantomData;
use rocket::http::RawStr;
use std::str::FromStr;


/// Template value arg, refer to [`HexString`](struct.HexString.html).
///
/// Rust doesn't have value template args so this is just one big one.
pub trait StringLength {
    /// The requested length of the string.
    const LENGTH: usize;
}

/// Character length of a SCrypt key with `dkLen=64`,
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SCrypt64Length;

impl StringLength for SCrypt64Length {
    const LENGTH: usize = 64 * 2;
}


/// Require that the string not be empty.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use sudoku_backend::ops::constraints::NonEmpty;
/// assert_eq!(NonEmpty::from_str("Давай пойдём в Москву!"),
///            Ok(NonEmpty("Давай пойдём в Москву!".to_string())));
///
/// assert!(NonEmpty::from_str("").is_err());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmpty(pub String);

/// Require that the string be hexadecimal and of the specified width.
///
/// The error type can be deconstructed as:
///   * `Some(Some(len))` ⇒ supplied string has invalid `len`gth
///   * `Some(None)` ⇒ supplied string contains non-hexadecimal characters
///   * `None` ⇒ supplied string contains non-hexadecimal characters
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use std::marker::PhantomData;
/// # use sudoku_backend::ops::constraints::{StringLength, HexString};
/// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// pub struct TestLength;
///
/// impl StringLength for TestLength {
///     const LENGTH: usize = 8;
/// }
///
/// assert_eq!(HexString::<TestLength>::from_str("0a1B3c4D"),
///            Ok(HexString("0a1B3c4D".to_string(), PhantomData)));
///
/// assert!(HexString::<TestLength>::from_str("0a1B3c4").is_err());
/// assert!(HexString::<TestLength>::from_str("0a1B3c4D5").is_err());
/// assert!(HexString::<TestLength>::from_str("Давай по").is_err());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexString<P: StringLength>(pub String, pub PhantomData<P>);

/// Weakly validate the string to be roughly e-mail-shaped.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use sudoku_backend::ops::constraints::EMail;
/// assert_eq!(EMail::from_str("nabijaczleweli@gmail.com"),
///            Ok(EMail("nabijaczleweli@gmail.com".to_string())));
///
/// assert!(EMail::from_str("Давай пойдём в Москву!").is_err());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EMail(pub String);

/// Weakly validate the int to be positive.
///
/// TODO: example
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Positive<T: FromStr + From<i32> + PartialOrd>(pub T);


impl<'v> FromFormValue<'v> for NonEmpty {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        match String::from_form_value(form_value).map_err(|_| ())?.trim() {
            "" => Err(()),
            v => Ok(NonEmpty(v.to_string())),
        }
    }
}

impl<'v, P: StringLength> FromFormValue<'v> for HexString<P> {
    type Error = Option<Option<usize>>;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let value = String::from_form_value(form_value).map_err(|_| {
                let e: Option<Option<usize>> = None;
                e
            })?;
        let value = value.trim();
        if value.len() == P::LENGTH {
            if !value.contains(|c| !"0123456789abcdefABCDEF".contains(c)) {
                Ok(HexString(value.to_string(), PhantomData))
            } else {
                Err(Some(None))
            }
        } else {
            Err(Some(Some(value.len())))
        }
    }
}

impl<'v> FromFormValue<'v> for EMail {
    type Error = Option<&'static str>;

    /// Any deeper kind of validation is either too complex or too bad to be reliable.
    ///
    /// This, while not providing much guarantee, at least enforces something resemblant of an e-mail address.
    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let value = String::from_form_value(form_value).map_err(|_| None)?;
        match value.trim() {
            "" => Err(Some("Empty string isn't an e-mail address")),
            v => {
                let mut it = v.rsplitn(2, '@');
                let domain = it.next();
                let username = it.next();
                match (username.is_some(), domain.is_some()) {
                    (false, false) => Err(Some("missing @")),
                    (true, false) => Err(Some("missing domain")),
                    (false, true) => Err(Some("missing username")),
                    (true, true) => Ok(EMail(v.to_string())),
                }
            }
        }
    }
}

impl<'v, T: FromStr + From<i32> + PartialOrd> FromFormValue<'v> for Positive<T> {
    type Error = Option<<T as FromStr>::Err>;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        match form_value.parse() {
            Ok(p) => {
                if p > T::from(0) {
                    Ok(Positive(p))
                } else {
                    Err(None)
                }
            }
            Err(e) => Err(Some(e)),
        }
    }
}

impl FromStr for NonEmpty {
    type Err = <Self as FromFormValue<'static>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_form_value(RawStr::from_str(s))
    }
}

impl<P: StringLength> FromStr for HexString<P> {
    type Err = <Self as FromFormValue<'static>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_form_value(RawStr::from_str(s))
    }
}

impl FromStr for EMail {
    type Err = <Self as FromFormValue<'static>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_form_value(RawStr::from_str(s))
    }
}

impl<T: FromStr + From<i32> + PartialOrd> FromStr for Positive<T> {
    type Err = <Self as FromFormValue<'static>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_form_value(RawStr::from_str(s))
    }
}
