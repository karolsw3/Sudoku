# Sudoku web app

## Introduction

The app is written in Vue using single-file-components as it is intended to be as modular as possible.

However, many components need to access the centralized store to read/write states such as board state, common public settings etc.
so the app is integrated with a Vuex store.

## Installation

First, you need to install the dependencies by
```
  npm install
```
And then, to run a local server:
```
  npm run serve
```
To build the app, simply run:
```
  npm run build
```

## The vuex store

The store is declared in main.js file and it consists of following states:
* <b>input[]</b>  
  Array. Stores values of all inputs in login and register form
  To access a specified input you need to know its ID:
  ```javascript
    // For inputs in a register form, a 'register__' prefix is used:
    store.input['register__username']
    // Other inputs respectively:
    store.input['login__password']
    store.input['register__email']
    // etc...
  ```
* <b>userLogged</b>  
  Boolean. True is a user in logged, false otherwise.
* <b>selectedSlot</b>  
  Object. Stores x and y coordinates of the selected slot on the board.
* <b>boardState</b>  
  9x9 matrix. Stores all slots of the board in a matrix, where a slot can contain a number between 0 and 9 (0 means an empty slot).
