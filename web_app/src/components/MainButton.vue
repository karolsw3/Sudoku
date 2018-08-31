<template lang="pug">
.MainButton(@click="$emit('clicked')" :disabled="disabled")
  slot
</template>

<script>

export default {
  name: 'MainButton',
  props: ['disabled']
}
</script>

<style scoped lang="stylus">
.MainButton
  display inline-block
  margin 5px 20px
  width 140px
  color white
  background #0037dd
  padding 7px 20px
  box-sizing border-box
  border none
  border-radius 3px
  font-size 16px
  font-family 'Open Sans'
  font-weight 700
  text-transform uppercase
  cursor pointer
  box-shadow inset 0 0 8px 4px #1349ff
  transition-duration .2s
  &:hover
    opacity 0.95
    background #1349ff
</style>
