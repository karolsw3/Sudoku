<template lang="pug">
  .play
    Board
      Timer
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
      this.$store.state.boardState[this.$store.state.selectedSlot.x][this.$store.state.selectedSlot.y] = number
    }
  },
  created () {
    switch (this.difficulty) {
      case 'easy':
        this.difficulty = 1
        break
      case 'medium':
        this.difficulty = 2
        break
      case 'hard':
        this.difficulty = 3
    }

    axios.get('https://api.myjson.com/bins/1cvca6') // Will be: '/api/generateBoard?difficulty=1&variant=0'
      .then((response) => {
        this.$store.commit('mutateBoard', response.data.board)
      })
      .catch(function (error) {
        console.error(error)
      })
  }
}
</script>
