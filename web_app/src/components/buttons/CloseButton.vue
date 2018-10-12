<template lang="pug">
.CloseButton(@click="$emit('clicked')" :disabled="disabled")
  slot
</template>

<script>

export default {
  name: 'CloseButton',
  props: ['disabled']
}
</script>

<style scoped lang="stylus">
.CloseButton
  position absolute
  top 15px
  right 15px
  display inline-block
  width 15px
  height 15px
  background-image url('../../../../images/icons/close.svg')
  background-size cover
  text-transform uppercase
  cursor pointer
  transition-duration .2s
  &:hover
    opacity 0.9
</style>
