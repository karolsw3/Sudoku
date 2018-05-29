use diesel::{self, ExpressionMethods, ExecuteDsl, FilterDsl};
use chrono::{NaiveDateTime, Duration, Utc};
use self::super::super::tables::sessions;
use diesel::sqlite::SqliteConnection;


/// Refer to `doc/session.md` for more details.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="sessions"]
pub struct Session {
    pub id: Option<i32>,
    pub expiry: NaiveDateTime,
    pub is_admin: bool,
    pub user_id: Option<i32>,

    // pub product_id: Option<i32>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            id: None,
            expiry: NaiveDateTime::from_timestamp((Utc::now() + Duration::days(1)).naive_utc().timestamp(), 0),
            is_admin: false,
            user_id: None,
            // product_id: None,
        }
    }

    // pub fn set_product(&mut self, pid: i32, db: &SqliteConnection) -> Result<(), &'static str> {
    // self.product_id = Some(pid);
    // diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_|
    // "update failed")
    // }

    pub fn log_in(&mut self, uid: i32, is_admin: bool, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = Some(uid);
        self.is_admin = is_admin;
        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    pub fn log_out(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = None;
        self.is_admin = false;
        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }
}
