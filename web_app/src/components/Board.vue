<template lang="pug">
.Board
  slot
  .Board__grid.Board__grid--main
    template(v-for="i in 3")
      .Board__grid(v-for="j in 3")
        template(v-for="x in 3")
          .Board__slot(
            v-for="y in 3"
            @click="onSlotClick(getSlotX(i, j, x, y), getSlotY(i, j, x, y))"
            :class="[getSelectedClass(i, j, x, y), getLockedClass(i, j, x, y)]"
          )
            p(v-if="boardState[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] != 0" ) {{boardState[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] % 10}}
</template>

<script>

export default {
  name: 'Board',
  data: function () {
    return {
      filledSlots: 0
    }
  },
  methods: {
    onSlotClick (x, y) {
      this.$store.commit('slotSelected', {x, y})
    },
    getSlotX (i, j, x, y) {
      return (i - 1) * 3 + (x - 1)
    },
    getSlotY (i, j, x, y) {
      return (j - 1) * 3 + (y - 1)
    },
    getSelectedClass (i, j, x, y) {
      return (i - 1) * 3 + (x - 1) === this.selectedSlot.x && (j - 1) * 3 + (y - 1) === this.selectedSlot.y ? 'Board__slot--selected' : ''
    },
    getLockedClass (i, j, x, y) {
      return this.boardState[(i - 1) * 3 + (x - 1)][(j - 1) * 3 + (y - 1)] > 10 ? 'Board__slot--locked' : ''
    },
    lockSlots () { // locks all currently filled slots
      for (let row in this.boardState) {
        for (let column in this.boardState) {
          if (this.boardState[row][column] > 0) {
            this.boardState[row][column] += 10 // If value is greater than > 10 it means that the slot is locked
          }
        }
      }
    },
    countFilledSlots () {
      for (let row in this.boardState) {
        for (let column in this.boardState[row]) {
          if (this.boardState[row][column] > 0) {
            this.$store.commit('incrementFilledSlotsCounter')
          }
        }
      }
    }
  },
  computed: {
    boardState () {
      return this.$store.state.boardState
    },
    selectedSlot () {
      return this.$store.state.selectedSlot
    }
  }
}
</script>

<style scoped lang="stylus">
.Board
  display inline-block
  position relative
  margin 110px auto 0 auto
  box-sizing border-box
  border-radius 5px
  &__grid
    position relative
    width 100%
    height 100%
    display grid
    background #f5f5f5
    box-sizing border-box
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    &--main
      width 404px
      height 404px
      background #f5f5f5
      border-radius 5px
      border 1px solid #f5f5f5
  &__slot
    background white
    box-sizing border-box
    width 100%
    height 100%
    max-height 100%
    max-width 100%
    text-align center
    font-weight 700
    color #0445b7
    cursor pointer
    transition-duration .2s
    &:hover
      background #e6e8eb
    &--selected
      background #0445b7 !important
      color white
    &--locked
      background #f5f5f5 !important
      color #222 !important
</style>
