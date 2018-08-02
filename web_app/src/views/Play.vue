<template lang="pug">
  .play
    Board(ref='board')
      Timer(ref='timer')
      NumberSelector(@numberSelected="numberSelected")
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
        let board = this.$refs.board
        let timer = this.$refs.timer
        board.slots = this.deserializeBoardSkeleton(response.data.board_skeleton)
        board.countFilledSlots()
        board.lockSlots()
        timer.start()
      })
      .catch((error) => {})
  }
}
</script>
