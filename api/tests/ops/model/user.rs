use chrono::{NaiveDateTime, Utc};
use sudoku_backend::ops::User;


#[test]
fn new() {
    assert_eq!(User::new("uname", "password", "email"),
               User {
                   id: None,
                   username: "uname".to_string(),
                   password: "password".to_string(),
                   email: "email".to_string(),
                   created_at: NaiveDateTime::from_timestamp(Utc::now().naive_utc().timestamp(), 0),
                   is_admin: false,
                   points_total: 0,
               });
}
