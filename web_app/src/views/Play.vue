<template lang="pug">
  .play
    .progressSpinner
      md-progress-spinner(v-if='loadingBoard' md-mode="indeterminate")
    Board(ref='board' v-on:board-is-valid="submitBoard")
      Timer(ref='timer')
      NumberSelector(@numberSelected='numberSelected')
</template>

<script>
import Board from '@/components/Board.vue'
import NumberSelector from '@/components/NumberSelector.vue'
import Timer from '@/components/Timer.vue'
import axios from 'axios'
import base64 from 'base-64'

export default {
  name: 'play',
  components: {
    Board, NumberSelector, Timer
  },
  props: ['difficulty'],
  data: function () {
    return {
      'loadingBoard': true,
      'boardId': 0,
      'boardSkeleton' : ''
    }
  },
  methods: {
    numberSelected (number) {
      let board = this.$refs.board
      board.mutateSelectedSlot(number)
    },
    deserializeBoardSkeleton (boardSkeleton) {
      let boardMatrix = Array(9).fill().map(() => Array(9).fill(0))
      boardSkeleton.split('').map((number, index) => {
        if (number === '.') number = 0
        else number = parseInt(number)
        boardMatrix[Math.floor(index / 9)][index % 9] = number
      })
      return boardMatrix
    },
    serializeBoardSkeleton (boardSkeleton) {
      let serializedBoard = ''
      let board = this.$refs.board
      for (let row in board.slots) {
        for (let column in board.slots) {
          if (board.slots[row][column] % 10 === 0) {
            serializedBoard += '3'
          } else {
            serializedBoard += board.slots[row][column] % 10
          }
        }
      }
      return serializedBoard
    },
    submitBoard (board) {
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/v1/play/submit', true)
      xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded')
      xhr.onload = (response) => {
        this.loading = false
        switch (response.target.status) {
          case 412:
            this.error = true
            this.errorMessage = 'Cheater!'
            break
          case 201:
            // Success
            let responseData = JSON.parse(response.target.response)
            console.log(responseData)
            break
        }
      }
      xhr.send('board_id=' + this.boardId + '&board_skeleton=' + this.boardSkeleton + '&solved_board=' + this.serializeBoardSkeleton(board))
    }
  },
  computed: {
    difficultyNumber () {
      switch (this.difficulty) {
        case 'easy':
          return 1
        case 'medium':
          return 2
        case 'hard':
          return 3
        default:
          return 1
      }
    }
  },
  created () {
    axios.get('/api/v1/play/new?difficulty=' + this.difficultyNumber)
      .then((response) => {
        console.log('Board loaded')
        let board = this.$refs.board
        let timer = this.$refs.timer
        board.slots = this.deserializeBoardSkeleton(response.data.board_skeleton)
        this.boardSkeleton = response.data.board_skeleton
        this.boardId = response.data.board_id
        board.countFilledSlots()
        board.lockSlots()
        this.loadingBoard = false
        timer.start()
      })
      .catch((error) => {
        console.error(error)
      })
  }
}
</script>

<style scoped lang="stylus">
.progressSpinner
  position fixed
  display inline-block
  margin calc(50vh - 155px) auto 0 auto
  left 0
  right 0
  z-index 999
</style>
