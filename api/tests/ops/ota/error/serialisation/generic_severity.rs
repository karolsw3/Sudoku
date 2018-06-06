use sudoku_backend::ops::errors::GenericErrorSeverity;
use serde_json;


#[test]
fn danger() {
    assert_eq!(serde_json::to_string_pretty(&GenericErrorSeverity::Danger).unwrap(), "\"danger\"");
}

#[test]
fn warning() {
    assert_eq!(serde_json::to_string_pretty(&GenericErrorSeverity::Warning).unwrap(), "\"warning\"");
}
