<template lang="pug">
  .play
    ProgressSpinner(v-if='loading')
    GameSummary(v-if='summary.show' :solutionDuration='summary.solutionDuration' :difficulty='summary.difficulty' :score='summary.score' v-on:summaryClosed='summary.show = false')
    Board(ref='board' v-on:board-is-valid="submitBoard")
      Timer(ref='timer')
      NumberSelector(@numberSelected='numberSelected')
</template>

<script>
import Board from '@/components/Board.vue'
import NumberSelector from '@/components/NumberSelector.vue'
import ProgressSpinner from '@/components/ProgressSpinner.vue'
import GameSummary from '@/components/GameSummary.vue'
import Timer from '@/components/Timer.vue'
import axios from 'axios'

export default {
  name: 'play',
  components: {
    Board, NumberSelector, Timer, GameSummary, ProgressSpinner
  },
  props: ['difficulty'],
  data: function () {
    return {
      loading: true,
      boardId: 0,
      boardSkeleton: '',
      summary: {
        show: false,
        solutionDuration: 0,
        difficulty: '',
        score: 0
      }
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
            serializedBoard += '.'
          } else {
            serializedBoard += board.slots[row][column] % 10
          }
        }
      }
      return serializedBoard
    },
    submitBoard (board) {
      this.loading = true
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/v1/play/submit', true)
      xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded')
      xhr.onload = (response) => {
        this.loading = false
        switch (response.target.status) {
          case 412:
            // That's the thing that shouldn't be
            break
          case 200:
            // Success
            let responseData = JSON.parse(response.target.response)
            this.summary = {
              show: true,
              solutionDuration: responseData.solution_duration_secs,
              difficulty: responseData.difficulty,
              score: responseData.score
            }
            this.$refs.timer.stop()
            this.$store.commit('addPoints', this.summary.score)
            this.$store.commit('addGames')
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
        let board = this.$refs.board
        let timer = this.$refs.timer
        board.slots = this.deserializeBoardSkeleton(response.data.board_skeleton)
        this.boardSkeleton = response.data.board_skeleton
        this.boardId = response.data.board_id
        board.countFilledSlots()
        board.lockSlots()
        this.loading = false
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
