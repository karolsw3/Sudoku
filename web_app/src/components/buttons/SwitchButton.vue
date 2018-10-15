<template lang="pug">
.SwitchButton(@click="$emit('clicked'); switched = !switched" :class='switched ? "SwitchButton--switched":""')
  slot
  .SwitchButton__tooltip {{tooltip}} {{switched ? "ON":"OFF"}}
</template>

<script>

export default {
  name: 'SwitchButton',
  data: function () {
    return {
      switched: false
    }
  },
  props: ['tooltip']
}
</script>

<style scoped lang="stylus">
.SwitchButton
  display inline-block
  margin 5px
  width 48px
  height 48px
  background #0037dd
  padding 7px 7px
  box-sizing border-box
  border none
  border-radius 4px
  font-size 16px
  cursor pointer
  box-sizing border-box
  transition-duration .2s
  filter hue-rotate(0deg)
  z-index 3
  &--switched
    filter hue-rotate(90deg)
  &:hover
    opacity .9
    .SwitchButton__tooltip
      display block
  img
    width 32px
    height 32px
  &__tooltip
    display none
    position absolute
    left -30px
    top -33px
    width 120px
    height auto
    border-radius 3px
    border 1px solid #333
    background #222
    color white
    cursor default
    font-size 12px
    font-family 'Arial', sans-serif
    padding 3px
    z-index 3
</style>
