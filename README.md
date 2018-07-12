# <img src="images/logo_big.png" height="150" alt="sudoku" />
A modern, API-based Sudoku app.

## Introduction

Our goal is to make the best Sudoku app on the web.

There are a lot of Sudoku games on the web, but none of them well built nor with good design.
We want to make our take modern, secure, well-designed and open to everyone – it works well on all available platforms and screens.

`Sudoku` consists of a public API, web app, and in the future – a mobile app.

# <img src="images/vectors/progress.svg" alt="Progress"/>

## API

<sub>Subject to change and move elsewhere with rest of API documentation.</sub>

  - USERS
    - Register a new user
      > POST `sudoku/api/register`
      ```javascript
        // Returned JSON
        'data': {
          'success': true,
          'message': 'New user registered successfully',
          'code': 201
        }
      ```
    - Validate users login and password
      > POST `sudoku/api/login`
      ```javascript
        // Returned JSON
        'data': {
          'success': false,
          'message': 'Incorrect password',
          'code': 401
        }
      ```
    - Return the number of boards solved by a user (ordered by duration and difficulty)
      > GET `sudoku/api/getScore?user=username`
      ```javascript
        // Returned JSON
        'data': {
          'games': [
            {
              'id': 452,
              'difficulty': 2,
              'duration': 23438
            } //, ...
          ]
        }
      ```
    - Compare users scores in a leaderboard
      > GET `sudoku/api/getLeaderboard?page=1`
      ```javascript
        // Returned JSON
        'data': {
          'page': 1,
          'maxPages': 34,
          'leaderboard': [
            {
              'position': 1,
              'username': 'John Smith',
              'points': 10987,
              'gamesPlayed': 546
            } //, ...
          ]
        }
      ```
  - GAME
    - Generate new sudoku boards
      > GET `sudoku/api/generateBoard?difficulty=1&variant=0`
      ```javascript
        // Returned JSON
        'data': {
          'board': [
            [5,3,0,0,7,0,0,0,0],
            [6,0,0,1,9,5,0,0,0],
            [0,9,8,0,0,0,0,6,0],
            [8,0,0,0,6,0,0,0,3],
            [4,0,0,8,0,3,0,0,1],
            [7,0,0,0,2,0,0,0,6],
            [0,6,0,0,0,0,2,8,0],
            [0,0,0,4,1,9,0,0,5],
            [0,0,0,0,8,0,0,7,9]
          ]
        }
      ```
    - Validate completed ones
      > POST `sudoku/api/validateBoard`
      ```javascript
        // Returned JSON
        'data': {
          'isValid': false,
          'message': 'It ain\'t so easy lil boy..'
        }
      ```

After a game is won, the player is awarded points, calculated as follows:

```c
points = difficulty * (3000 / solvingDuration + 30)
```

Where `difficulty` ϵ {1, 2, 3} and `solvingDuration` is in seconds.

All endpoint data is returned in  **JSON**.

## Database

<sub>Here temporarily, subject to change and move elsewhere with rest of internal design documents.</sub>

### USERS
|  id |    name    |      password     |      seed      |          email         |  role | points |
|-----|------------|-------------------|----------------|------------------------|-------|-------:|
| `0` |  karolsw3  | 14c80afe290ba6dE1 | 4FaCc948fab1B2 | karol.sw3@gmail.com    | admin |    983 |
| `1` |     bob    | 44f80cfeC53Aa4d71 | 911Cd9D82abeC5 | bob@blob.com           | user  |   3984 |
| `2` | sudokuPapa | 5ff34cac003ca4c90 | 3FFaDa3fe8be47 | noobfrom@minecraft.net | user  |  45682 |
etc.

### GAMES

|  id | userId | difficulty | duration |    date    |
|-----|-------:|-----------:|---------:|------------|
| `0` |    0   |      1     |    325   | 21-04-2018 |
| `1` |    0   |      1     |    315   | 21-04-2018 |
| `2` |    2   |      1     |     62   | 12-06-2018 |
| `3` |    1   |      2     |    142   | 03-12-2017 |
etc.

## Contributing

We are open to contributors, so don't hesitate to make a pull request!

## Licence

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details
