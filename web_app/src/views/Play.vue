<template lang="pug">
  .play
    Board
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
  computed: {
    boardState () {
      return this.$store.state.boardState
    }
  },
  methods: {
    numberSelected (number) {
      this.$store.commit('mutateBoardSlot', {
        x: this.$store.state.selectedSlot.x,
        y: this.$store.state.selectedSlot.y,
        value: number
      })
    },
    lockSlots () { // locks all currently filled slots
      for (let row in this.boardState) {
        for (let column in this.boardState) {
          if (this.boardState[row][column] > 0) {
            this.boardState[row][column] += 10 // If value is greater than > 10 it means that the slot is locked
          }
        }
      }
    }
  },
  created () {
    axios.get('https://api.myjson.com/bins/1cvca6') // Will be: '/api/generateBoard?difficulty=1&variant=0'
      .then((response) => {
        this.$store.commit('mutateBoard', response.data.board)
        this.lockSlots()
        let timer = this.$refs.timer
        timer.start()
      })
      .catch(function (error) {
        console.error(error)
      })
  }
}
</script>
