use sudoku_backend::ops::LoginError;
use serde_json;


#[test]
fn login() {
    assert_eq!(serde_json::from_str::<LoginError>("{}").unwrap(), LoginError {});
}
