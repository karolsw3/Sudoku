# User list
Users contain identifying data, status (admin/not), and points.

## Authentication
### Client-side
When the user presses "Log in" on the login page, the plaintext password is key-derived with [`scrypt`](https://github.com/ricmoo/scrypt-js).

The entered password is not normalised in any way. The arguments passed to the client-side `scrypt` function are

parameter |      value     | comment                                                                                                              |
----------|----------------|----------------------------------------------------------------------------------------------------------------------|
`N`       | 2<sup>14</sup> | Recommended for passwords                                                                                            |
`r`       | 8              | "Optimum value" specified in [paper](http://www.tarsnap.com/scrypt/scrypt.pdf) (page 12)                             |
`p`       | 1              | "Optimum value" specified in [paper](http://www.tarsnap.com/scrypt/scrypt.pdf) (page 12)                             |
`dkLen`   | 64             |                                                                                                                      |
`salt`    | "Sudoku"       | That's what the project is called, and, honestly, it doesn't need to be *that* secure, just preferably not plaintext |

Nota bene: these values **must** be consistent once and for all, as other values will create different hashes, which will in turn create different passwords.

That value is then hex-string-encoded (case irrelevant), a JSON-stringified form is constructed, then base64-encoded as "data" and that is sent in a form.

In other words, with `data` being the key and value, and doubling as <span id="user-login-data">User Login Data</span> (all keys `string`s):
```js
let data = base64(JSON.stringify({
    username: raw_username,
    email: raw_email, // See note below
    password: scrypt(raw_password, /* With ^ params */)
}));
```

The `email` key *shall only be present* for user registration – the form will be rejected if it contains an email and is used to log in;
  *vice versa* – the registration request will be rejected if it doesn't contain the `email` key.

### Server-side verification
If a user with the specified username exists in the database,
the server lower-cases the key-derived password, then compares it to the stored value.

Otherwise, the server shall return the same value as with above with unmatched password.

The returned status shall be `202 Accepted` on correct verification
                         and `401 Unauthorized` otherwise.

The returned value shall be [Sanitised User data](#sanitised-user-data) on correct verification,
                        and [Login Error](errors.md#login-error) otherwise.

The returned bundle shall *not* distinguish between any two cases of incorrect verification.

### Server-side entry creation
If a user with the specified username or e-mail doesn't exist in the database,
the server lower-cases the key-derived password, then derives it *again* with parameters as guessed via `util::SCRYPT_IDEAL_PARAMS`.
Finally, the server stores that username, e-mail, and
doubly-derived password in the [`rscrypt format`](https://docs.rs/rust-crypto/0.2.36/crypto/scrypt/fn.scrypt_simple.html#format) in the `users` table.

The returned status shall be `201 Created` on correct creation,
                             `409 Conflict` if a user with that name/e-mail already exists,
                         and an otherwise implementation-defined relevant status on other errors.

The returned value shall be [Sanitised User data](#sanitised-user-data) on correct creation,
                         an appropriately filled out [Generic Error](errors.md#generic-error) if a user with that name/e-mail already exists,
                     and an otherwise implementation-defined relevant [Error](errors.md) on other errors.

### Logging out
Independently of whether the user is logged in, the server shall return `204 No Content`.

## SQL table def

```sql
CREATE TABLE IF NOT EXISTS users (
    id           INTEGER PRIMARY KEY ASC,     -- Unique user ID
    username     TEXT NOT NULL UNIQUE,        -- User's name or "login" or whatever you want to call it
    password     TEXT NOT NULL,               -- Doubly scrypted password text, see above.
    email        TEXT NOT NULL UNIQUE,        -- User's contact e-mail
    created_at   DATETIME NOT NULL,           -- Time user was created
    is_admin     BOOLEAN NOT NULL DEFAULT 0,  -- Whether the user has administrative privileges
    points_total INTEGER NOT NULL DEFAULT 0,  -- Sum total of the user's points, calculated according to sudoku.md#scoring-formula, non-negative
    games_total  INTEGER NOT NULL DEFAULT 0,  -- Amount of games played, non-negative

    CHECK ((points_total >= 0) AND (games_total >= 0))
);
```

## Sanitised User data

```json
{
    "username":     "string",
    "created_at":   "RFC3339 (string)",
    "is_admin":     "boolean",
    "points_total": "number",
    "games_total":  "number"
}
```

## Admins

Admins are assigned manually, where `$1` is the username whose adminness to set:

<!-- no_run -->

```sql
UPDATE users
    SET is_admin = 1
    WHERE username = "$1";
```

Other keys will of course, per analogiam, work.
