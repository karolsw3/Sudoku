use sudoku_backend::ops::errors::{GenericErrorSeverity, GenericError};
use serde_json;


#[test]
fn danger() {
    assert_eq!(serde_json::to_string_pretty(&GenericError {
                       reason: "failed to commit sudoku".into(),
                       severity: GenericErrorSeverity::Danger,
                   })
                   .unwrap(),
               r#"{
  "reason": "failed to commit sudoku",
  "severity": "danger"
}"#);
}

#[test]
fn warning() {
    assert_eq!(serde_json::to_string_pretty(&GenericError {
                       reason: "failed to commit sudoku".into(),
                       severity: GenericErrorSeverity::Warning,
                   })
                   .unwrap(),
               r#"{
  "reason": "failed to commit sudoku",
  "severity": "warning"
}"#);
}
