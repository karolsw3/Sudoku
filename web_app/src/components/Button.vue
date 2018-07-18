<template lang="pug">
button.Button(@click="$emit('clicked')")
  slot
</template>

<script>
export default {
  name: 'Button'
}
</script>

<style scoped lang="stylus">
.Button
  width 100%
  box-sizing content-box
  max-width 400px
  padding 19px 20px
  margin-top 15px
  background #123ffc
  color white
  border none
  border-radius 2px
  transition-duration 0.2s
  cursor pointer
  &:hover
    opacity 0.8
  &--disabled
    background #dedede
    color #aaa
    cursor default
</style>
