use sudoku_backend::ops::LoginError;
use serde_json;


#[test]
fn login() {
    assert_eq!(serde_json::to_value(LoginError {}).unwrap(), json!({}));
}
