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
    submitBoard () {
      /* let data = {
        boardState: this.$store.boardState
      }
       axios.post('/api/validateBoard', data)
      ... then */
    }
  },
  created () {
    axios.get('https://api.myjson.com/bins/1cvca6') // Will be: '/api/generateBoard?difficulty=1&variant=0'
      .then((response) => {
        this.$store.commit('mutateBoard', response.data.board)
        let board = this.$refs.board
        let timer = this.$refs.timer
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
