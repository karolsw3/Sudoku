# Sudoku
<img src="https://image.ibb.co/gHVBAT/sudoku.png" height="150"><br>
A modern, API-based sudoku app.

## Introduction

Our goal is to make the best sudoku app on the web.

There are a lot of sudoku games on the web, but none of them is well built nor has a good design.
We want to make our version modern, secure, well-designed and open to everyone. It should work on all available platforms and screens.

The Sudoku project will consist of a public API, web app, and in the future - a mobile app.

## API

The API will be used both by web and mobile app.
It should:
  - USERS
    - Register a new user
      > POST ``sudoku/register``
    - Validate users login and password
      > POST ``sudoku/login``
    - Return the number of boards solved by a user (ordered by duration and difficulty) 
      > GET ``sudoku/getScore?user=username``
    - Compare users scores in a leaderboard
      > GET ``sudoku/getLeaderboard``
  - GAME
    - Generate new sudoku boards
      > GET ``sudoku/generateBoard?difficulty=1&variant=0``
    - Validate completed ones
      > POST ``sudoku/validateBoard``

<br>
After each game validation the server should give points to the player, calculated as follows:

> points = difficulty * (3000 / solvingDuration + 30)

Where difficulty ϵ {1, 2, 3} and solvingDuration is in seconds.

All data should be returned in the <b>JSON</b> format.

## DATABASE

### USERS
| id | name | password | seed | email | role | points |
|---|---|---|---|---|---|---|
|``0``|karolsw3|14c80afe290ba6dE1|4FaCc948fab1B2|karol.sw3@gmail.com|admin|983|
|``1``|bob|44f80cfeC53Aa4d71|911Cd9D82abeC5|bob@blob.com|user|3984|
|``2``|sudokuPapa|5ff34cac003ca4c90|3FFaDa3fe8be47|noobfrom@minecraft.net|user|45682|
etc..

### GAMES

| id | userId | difficulty | duration | date |
|---|---|---|---|---|
|``0``|0|1|325|21-04-2018|
|``1``|0|1|315|21-04-2018|
|``2``|2|1|62|12-06-2018|
|``3``|1|2|142|03-12-2017|
etc..

## Contributing

We are open for contributors. If you want to make a pull request - don't hesitate.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
