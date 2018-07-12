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

export default {
  name: 'play',
  components: {
    Board, NumberSelector, Timer
  },
  props: ['level'],
  methods: {
    numberSelected (number) {
      this.$store.state.boardState[this.$store.state.selectedSlot.x][this.$store.state.selectedSlot.y] = number
    }
  }
}
</script>
