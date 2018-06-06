use sudoku_backend::ops::errors::GenericErrorSeverity;
use serde_json;


#[test]
fn unrecognised() {
    assert_eq!(serde_json::from_str::<GenericErrorSeverity>("\"benlo\"").unwrap_err().to_string(),
               "unknown variant `benlo`, expected `warning` or `danger` at line 1 column 7");
}
