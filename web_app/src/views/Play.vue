<template lang="pug">
  .play
    .progressSpinner
      md-progress-spinner(v-if='loadingBoard' md-mode="indeterminate")
    Board(ref='board')
      Timer(ref='timer')
      NumberSelector(@numberSelected='numberSelected')
</template>

<script>
import Board from '@/components/Board.vue'
import NumberSelector from '@/components/NumberSelector.vue'
import Timer from '@/components/Timer.vue'
import axios from 'axios'

export default {
  name: 'play',
  components: {
    Board, NumberSelector, Timer
  },
  props: ['difficulty'],
  data: function () {
    return {
      'loadingBoard': true
    }
  },
  methods: {
    numberSelected (number) {
      let board = this.$refs.board
      board.mutateSelectedSlot(number)
    },
    deserializeBoardSkeleton (boardSkeleton) {
      let boardMatrix = Array(9).fill().map(() => Array(9).fill(0))
      boardSkeleton.split("").map((number, index) => {
        if (number === '.') number = 0
        else number = parseInt(number)
        boardMatrix[Math.floor(index / 9)][index % 9] = number
      })
      return boardMatrix
    }
  },
  computed: {
    difficultyNumber () {
      switch (this.difficulty) {
        case 'easy':
          return 1
          break
        case 'medium':
          return 2
          break
        case 'hard':
          return 3
          break
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
