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
    }
  },
  created () {
    axios.get('https://api.myjson.com/bins/1cvca6') // Will be: '/api/generateBoard?difficulty=1&variant=0'
      .then((response) => {
        let board = this.$refs.board
        let timer = this.$refs.timer
        board.slots = response.data.board
        board.countFilledSlots()
        board.lockSlots()
        timer.start()
      })
      .catch(function (error) {
        console.error(error)
      })
  }
}
</script>
