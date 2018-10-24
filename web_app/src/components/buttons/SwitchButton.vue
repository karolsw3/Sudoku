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
  width 42px
  height 42px
  background #aaa
  padding 7px 7px
  box-sizing border-box
  border-radius 4px
  font-size 16px
  cursor pointer
  box-sizing border-box
  transition-duration .2s
  filter hue-rotate(0deg)
  z-index 4
  &--switched
    background #0037dd
  &:hover
    opacity .95
    .SwitchButton__tooltip
      display block
  img
    width 26px
    height 28px
  &__tooltip
    display none
    position absolute
    left -35px
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
    z-index 4
</style>
