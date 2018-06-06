mod generic_severity;
mod generic;

use sudoku_backend::ops::errors::LoginError;
use serde_json;


#[test]
fn login() {
    assert_eq!(serde_json::to_string_pretty(&LoginError {}).unwrap(), "{}");
}
