//! `diesel` table definitions, copied over from `doc/`umentation.


table! {
    /// See `doc/user.md`
    users {
        /// Nullable wrapper so we can pass NULL to SQLite so it assigns new id
        id           -> Nullable<Integer>,
        username     -> Text,
        password     -> Text,
        email        -> Text,
        created_at   -> Timestamp,
        is_admin     -> Bool,
        points_total -> Integer,
    }
}
