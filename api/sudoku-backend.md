sudoku-backend(1) -- Back-end of the "modern, API-based sudoku app"
===================================================================

## SYNOPSIS

`sudoku-backend` [DATABASE_FILE] [LEADERBOARD_SETTINGS_FILE] [ACTIVITY_TIMEOUT]

## DESCRIPTION

The back-end of the "modern, API-based sudoku app",
separate from the server for the front-end.

This document is only concerned with configuring the behaviour
of the "game"/"server" layer. For configuring the hosting layer, consult
[the Rocket.rs guide](https://rocket.rs/guide/configuration).

## OPTIONS

   [DATABASE_FILE]

    The file in which to store the database, parent directory must exist.

    Default: ./sudoku-backend.db.

  [LEADERBOARD_SETTINGS_FILE]

    Optional file from which to load the leaderboard settings.

    Consult LEADERBOARD SETTINGS for more details

    If not present, defaults to ./leaderboard.toml, if exists,
    then to builtin defaults.

  [ACTIVITY_TIMEOUT]

    Amount of time in milliseconds after last request a session/user is considered
    to have "left the site".

    Default: 10'000.

## LEADERBOARD SETTINGS

The leaderboard settings are stored in TOML format, like so
(this configuration is also the defaults):

```toml
[board.default]
count = 10
ordering = 'best_to_worst'

[board.max]
count = 42
ordering = 'best_to_worst'


[person.default]
count = 3
ordering = 'best_to_worst'

[person.max]
count = 10
ordering = 'best_to_worst'
```

Both `default` and `max` in both sexions have the same format:

  * `count` – positive integer – how many entries to return
  * `ordering` – enum, either `'best_to_worst'`, or `'worst_to_best'` –
                 which results to obtain, and how to sort them afterward

In the `max` sexion, the `ordering` key is unused (but must still be valid).

The `board` sexion specifies configuration for leaderboards of solutions,
the `person` one – for those of registered in players.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/Galactim/Sudoku/issues>&gt;

## SEE ALSO

&lt;<https://github.com/Galactim/Sudoku>&gt;
