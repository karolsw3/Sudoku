<template lang="pug">
  .home
    NumberSelector(@numberSelected="numberSelected")
    Board()
</template>

<script>
import Board from '@/components/Board.vue'
import NumberSelector from '@/components/NumberSelector.vue'

export default {
  name: 'home',
  components: {
    Board, NumberSelector
  },
  methods: {
    numberSelected (number) {
      this.$store.state.boardState[this.$store.state.selectedSlot.x][this.$store.state.selectedSlot.y] = number
    }
  }
}
</script>
