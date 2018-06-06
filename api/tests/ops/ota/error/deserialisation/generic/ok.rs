use sudoku_backend::ops::errors::{GenericErrorSeverity, GenericError};
use serde_json;


#[test]
fn with_warning() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{
        "reason": "failed to commit sudoku",
        "severity": "warning"
    }"#)
                   .unwrap(),
               GenericError {
                   reason: "failed to commit sudoku".into(),
                   severity: GenericErrorSeverity::Warning,
               });
}

#[test]
fn with_danger() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{
        "reason": "failed to commit sudoku",
        "severity": "danger"
    }"#)
                   .unwrap(),
               GenericError {
                   reason: "failed to commit sudoku".into(),
                   severity: GenericErrorSeverity::Danger,
               });
}
