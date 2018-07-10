<template lang="pug">
.NumberSelector
  .NumberSelector__slot(v-for="n in 9" @click="$emit('numberSelected', n)") {{n}}
</template>

<script>
export default {
  name: 'NumberSelector'
}
</script>

<style scoped lang="stylus">
.NumberSelector
  position absolute
  bottom -60px
  left 0
  width 100%
  height 40px
  display flex
  &__slot
    width 100%
    height 100%
    border 2px solid #eee
    margin 0 5px
    color #777
    font-size 16px
    border-radius 5px
    display flex
    justify-content center
    align-items center
    cursor pointer
    box-sizing border-box
    transition-duration .12s
    &:hover
      background #0445b7
      border-color #0445b7
      color white
</style>
