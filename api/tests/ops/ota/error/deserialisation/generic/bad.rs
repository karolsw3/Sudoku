use sudoku_backend::ops::errors::GenericError;
use serde_json;


#[test]
fn incorrect_severity() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{
        "reason": "failed to commit sudoku",
        "severity": "benlo"
    }"#)
                   .unwrap_err()
                   .to_string(),
               "unknown variant `benlo`, expected `warning` or `danger` at line 3 column 27");
}

#[test]
fn missing_reason() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{
        "severity": "danger"
    }"#)
                   .unwrap_err()
                   .to_string(),
               "missing field `reason` at line 3 column 5");
}

#[test]
fn missing_severity() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{
        "reason": "failed to commit sudoku"
    }"#)
                   .unwrap_err()
                   .to_string(),
               "missing field `severity` at line 3 column 5");
}

#[test]
fn empty() {
    assert_eq!(serde_json::from_str::<GenericError>(r#"{}"#).unwrap_err().to_string(),
               "missing field `reason` at line 1 column 2");
}
