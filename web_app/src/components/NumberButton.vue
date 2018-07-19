<template lang="pug">
.NumberButton(:class='{"NumberButton--primary" : isPrimary}')
  slot
</template>

<script>
export default {
  name: 'NumberButton',
  props: ['isPrimary']
}
</script>

<style scoped lang="stylus">
.NumberButton
  width 34px
  height 34px
  border 1px solid #eee
  margin 0 3px
  color #777
  font-size 16px
  border-radius 5px
  display flex
  justify-content center
  align-items center
  cursor pointer
  box-sizing border-box
  text-decoration none
  transition-duration .12s
  &:hover
    background #123ffc
    border-color #123ffc
    color white
  &--primary
    border-color transparent
    background #ff4455
    color white
</style>
