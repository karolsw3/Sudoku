<template lang="pug">
  .play
    ProgressBar(v-if='loading')
    GameSummary(v-if='summary.show' :solutionDuration='summary.solutionDuration' :difficulty='summary.difficulty' :score='summary.score' v-on:summaryClosed='summary.show = false' v-on:reload='reload')
    Board(ref='board' v-on:board-is-valid="submitBoard" :pencilMode='pencilMode')
      Timer(ref='timer')
      NumberSelector(@numberSelected='numberSelected')
      SwitchButton.PencilModeSwitch(tooltip='Pencil mode' v-on:clicked='pencilMode = !pencilMode')
        img(src='../../../images/icons/pencil.svg')
</template>

<script>
import Board from '@/components/Board.vue'
import NumberSelector from '@/components/NumberSelector.vue'
import ProgressBar from '@/components/ProgressBar.vue'
import GameSummary from '@/components/GameSummary.vue'
import SwitchButton from '@/components/buttons/SwitchButton.vue'
import Timer from '@/components/Timer.vue'
import axios from 'axios'

export default {
  name: 'play',
  components: {
    Board, NumberSelector, Timer, GameSummary, ProgressBar, SwitchButton
  },
  props: ['difficulty'],
  data: function () {
    return {
      loading: true,
      boardId: 0,
      boardSkeleton: '',
      pencilMode: false,
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
      board.checkIfSlotHasBeenFilled(number)
      board.mutateSelectedSlot(number)
      board.checkIfBoardIsFullyFilled()
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
            this.$store.commit('addGames', this.summary.difficulty)
            break
        }
      }
      xhr.send('board_id=' + this.boardId + '&board_skeleton=' + this.boardSkeleton + '&solved_board=' + this.serializeBoardSkeleton(board))
    },
    reload () {
      let timer = this.$refs.timer
      timer.stop()
      this.newGame()
    },
    newGame () {
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
    this.newGame()
  }
}
</script>

<style scoped lang="stylus">
.play
  position relative
  margin 0 auto
  display flex
  justify-content center
  height 100%
  width 100%
  padding-bottom 100px
  align-items center
.PencilModeSwitch
  position absolute
  right -64px
  top 10px

@media screen and (max-width: 690px)
  .PencilModeSwitch
    position absolute
    right 10px
    top -56px
</style>
