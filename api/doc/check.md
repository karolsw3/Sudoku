# Data extraxion
Getting data out of the system

## Configuration
### Leaderboard

Both the defaults and maxes are changeable from the defaults presented below via the command line.

```toml
[default]
count = 10
ordering = 'best_to_worst'

[max]
count = 42
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

## Retrieval
### Leaderboard

The request query is a Loose Leaderboard Config.

That config is then clamped to the loaded max.

The response is an array of [Solution messages](sudoku.md#solution-message).
