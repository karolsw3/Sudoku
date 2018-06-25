# Sudoku

A modern, API-based sudoku web app.

## Introduction

Our goal is to renew the good name of sudoku.
We want to make it modern, secure and open to everyone.

The Sudoku will consist of public API, web app, and in the future - a mobile app.

## API

The API will be used both by web and mobile app.
It should:
  - USERS
    - Validate users logins and passwords
    - Return the number of boards solved by the user (ordered by time and difficulty)
    - Compare users scores in a leaderboard
  - GAME
    - Generate new sudoku boards
    - Validate completed ones
    - Calculate the time during which the user has solved the board
  
All data should be returned in the JSON format.

## Contributing

We are open for contributors. If you want to make a pull request - don't hesitate.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
