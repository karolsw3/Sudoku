use sudoku_backend::ops::errors::GenericErrorSeverity;
use serde_json;


#[test]
fn warning() {
    assert_eq!(serde_json::from_str::<GenericErrorSeverity>("\"warning\"").unwrap(), GenericErrorSeverity::Warning);
}

#[test]
fn danger() {
    assert_eq!(serde_json::from_str::<GenericErrorSeverity>("\"danger\"").unwrap(), GenericErrorSeverity::Danger);
}
