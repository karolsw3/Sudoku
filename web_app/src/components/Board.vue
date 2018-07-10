<template lang="pug">
.Board
  slot
  .Board__grid.Board__grid--main
    template(v-for="i in 3")
      .Board__grid(v-for="j in 3")
        template(v-for="x in 3")
          .Board__slot(v-for="y in 3" @click="onSlotClick((i - 1)*3 + (x - 1),(j - 1)*3 + (y - 1))" :class="{'Board__slot--selected' : (i - 1)*3 + (x - 1) == selectedSlot.x && (j - 1)*3 + (y - 1) == selectedSlot.y}")
            p(v-if="boardState[(i - 1)*3 + (x - 1)][(j - 1)*3 + (y - 1)] != 0" ) {{boardState[(i - 1)*3 + (x - 1)][(j - 1)*3 + (y - 1)]}}
</template>

<script>

export default {
  name: 'Board',
  methods: {
    onSlotClick (x, y) {
      this.$store.commit('slotSelected', {x, y})
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
  width 400px
  height 403px
  position relative
  margin 150px auto 0 auto
  border 2px solid #e6e8eb
  border-radius 5px
  &__grid
    position relative
    width 100%
    height 100%
    display grid
    background #f5f5f5
    grid-gap 2px
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    &--main
      height 400px
      background #e6e8eb
      grid-gap 2px
  &__slot
    background white
    box-sizing border-box
    width 100%
    height 100%
    text-align center
    grid-gap 1px
    font-weight 700
    color #0445b7
    cursor pointer
    transition-duration .2s
    &:hover
      background #e6e8eb
    &--selected
      background #0445b7 !important
      color white
</style>
