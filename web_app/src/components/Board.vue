<template lang="pug">
.Board
  .Board__grid.Board__grid--main
    .Board__grid(v-for="n in 9")
      .Board__slot(v-for="n in 9")
</template>

<script>
export default {
  name: 'Board'
}
</script>

<style scoped lang="stylus">
.Board
  display inline-block
  width 400px
  height 400px
  position relative
  margin 150px auto 0 auto
  border 2px solid #e6e8eb
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
    width 100%
    height 100%
    text-align center
    grid-gap 1px
    font-weight 700
    color #0445b7
</style>
