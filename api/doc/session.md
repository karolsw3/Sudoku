# Session management
Cookie encrypted with secure key [as per `rocket` spec](https://api.rocket.rs/rocket/http/enum.Cookies.html#method.add_private).

Cookie data is an `INTEGER`, which is a `PRIMARY KEY` in the `sessions` table.

Cookie expiry date is set to 1 day in the future in UTC as determined by `chrono::Utc::now() + chrono::Duration::days(1)` without sub-second precision.

Cookie name is `"session_id"`.

See [`user.md`](user.md) for user and login details.

## SQL table def

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id         INTEGER PRIMARY KEY ASC,           -- Unique session ID
    expiry     DATETIME NOT NULL,                 -- Expiry datetime in RFC3339 format
    is_admin   BOOLEAN NOT NULL DEFAULT 0,        -- Whether the user has authenticated as administrator
    user_id    INTEGER REFERENCES users (id)      -- ID of user session is logged in as

    -- TODO: add reference IDs to actual data (current-played board, etc.)
    -- product_id INTEGER REFERENCES products (id),  -- ID of product the user selected
);
```
