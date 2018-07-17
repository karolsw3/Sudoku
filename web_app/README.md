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
* <b>userLogged</b>  
  Boolean. True is a user in logged, false otherwise.

