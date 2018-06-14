# Sudoku processing
How is sudoku formed?

## Relevant data

A Sudoku board consists of the following:

```sql
CREATE TABLE IF NOT EXISTS sudoku_boards (
    id            INTEGER PRIMARY KEY ASC,                                               -- Unique board ID
    full_board    TEXT NOT NULL UNIQUE,                                                  -- The full "solved" board repr
    difficulty    INTEGER NOT NULL,                                                      -- Board "difficulty", between one and three
    creation_time DATETIME NOT NULL,                                                     -- Time the board was generated

    CHECK (((difficulty >= 1) AND (difficulty <= 3)) AND (LENGTH(full_board) == 9 * 9))
);
```

### Scoring formula

After game validates, points are awarded accordingly:

```c
point_count = difficulty * (30 + (3000 / solve_time))
```

Where `solve_time` is in seconds (but without the unit) and `difficulty ϵ {1, 2, 3}`.

## Board transserialisation

Given the following sudoku board:

```plaintext
5 | 3 | 4 || 6 | 7 | 8 || 9 | 1 |
6 | 7 |   || 1 | 9 | 5 || 3 | 4 | 8
1 | 9 | 8 || 3 | 4 | 2 || 5 | 6 | 7
———————————————————————————————————
8 | 5 | 9 || 7 | 6 | 1 || 4 | 2 | 3
4 |   | 6 ||   | 5 | 3 || 7 |   | 1
7 | 1 | 3 || 9 | 2 | 4 || 8 | 5 | 6
———————————————————————————————————
9 | 6 | 1 || 5 | 3 | 7 || 2 | 8 | 4
  | 8 | 7 || 4 |   | 9 || 6 | 3 | 5
3 | 4 | 5 || 2 | 8 | 6 ||   | 7 | 9
```

Equivalently:

```plaintext
53467891.
67.195348
198342567
859761423
4.6.537.1
713924856
961537284
.874.9635
345286.79
```

Effectively:

```plaintext
53467891.67.1953481983425678597614234.6.537.1713924856961537284.874.9635345286.79
```

## Workflow

Au première, get the skeleton board with of your preferred difficulty by specifying `?difficulty=<diff>`, where `diff` is within [`difficulty`'s domain](#scoring-formula).

Thereafter, submit the solved board as seen below.

### Board message

```json
{
  "board_id": "number",
  "board_skeleton": "string, last form under #board-transserialisation",

  "solved_board": "string, last form under #board-transserialisation"  // Only present when submitting a board solve
}
```

## Leaderboards

Or, well, just a list of solutions, because what's the difference.

```sql
CREATE TABLE IF NOT EXISTS sudoku_solutions (
    id                     INTEGER PRIMARY KEY ASC,                                                     -- Unique solution ID
    display_name           TEXT NOT NULL,                                                               -- Solver's display name
    board_id               INTEGER NOT NULL REFERENCES sudoku_boards (id),                              -- The solved board ID
    skeleton               TEXT NOT NULL,                                                               -- The solved board skeleton
    difficulty             INTEGER NOT NULL,                                                            -- Board "difficulty", between one and three
    solution_duration_secs INTEGER NOT NULL,                                                            -- Time in seconds taken to achieve the solution
    score                  INTEGER NOT NULL,                                                            -- Score achieved for the solve
    solution_time          DATETIME NOT NULL,                                                           -- Time the solution occured at

    CHECK (((difficulty >= 1) AND (difficulty <= 3)) AND (solution_duration_secs > 0) AND (score > 0))
);
```
