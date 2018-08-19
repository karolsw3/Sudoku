use serde_json::{self, Value as JsonValue};
use rocket::request::FromFormValue;
use rocket::http::RawStr;


/// How to order a solution list.
///
/// Refer to [`doc/check.rs`](../doc/check/) for more details.
///
/// The string reprs are `snake_case`.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum SolutionOrdering {
    BestToWorst,
    WorstToBest,
}

impl SolutionOrdering {
    /// The default ordering, for generic code.
    #[allow(non_upper_case_globals)]
    pub const Default: SolutionOrdering = SolutionOrdering::BestToWorst;
}

impl<'v> FromFormValue<'v> for SolutionOrdering {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        serde_json::from_value(JsonValue::String(form_value.to_string())).map_err(|_| ())
    }
}
