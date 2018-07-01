# Data extraxion
Getting data out of the system

## Configuration
### Leaderboard

Both the defaults and maxes are changeable from the defaults presented below via the command line.

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

Both `default` and `max` are a [Strict Leaderboard Config](#leaderboard-config).

#### Leaderboard Config

The Leaderboard Config consists of two keys:
  * `count` – `usize` – how many entries to return
  * `ordering` – [`SolutionOrdering`](#solution-ordering) – how to order the returned entries

Leaderboard Configs are said to be Strict if they require all keys to be present.
Otherwise they are Loose, and unspecified keys are filled in from the loaded defaults.

#### Solution Ordering

Any of:
  * `best_to_worst`
  * `worst_to_best`

#### Leaderboard Selector

Any of:
  * `solutions`
  * `users`

## Retrieval
### Leaderboard

The request query is a Loose Leaderboard Config, with another optional `of` key of type [Leaderboard Selector](#leaderboard-selector).

That config is then clamped to the loaded max.

Depending on the `of` key value (or `solutions` by default), the response is an array of:
  * [Solution messages](sudoku.md#solution-message), if `solutions`, or
  * [Sanitised user data](user.md#sanitised-user-data), if `users`.
