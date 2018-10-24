<template lang="pug">
.MainButton(:class="primary ? 'MainButton--primary':''")
  .MainButton__clicker(@click="$emit('clicked')")
    slot
</template>

<script>

export default {
  name: 'MainButton',
  props: ['primary']
}
</script>

<style scoped lang="stylus">
.MainButton
  display inline-block
  margin 0 20px
  padding 0
  color #aaa
  box-sizing border-box
  border none
  cursor default
  font-size 14px
  font-family 'Open Sans', sans-serif
  font-weight 100
  text-transform uppercase
  &--primary
    color #0037dd
  &__clicker
    cursor pointer
    transition-duration .15s
    &:hover
      opacity 0.6
</style>
