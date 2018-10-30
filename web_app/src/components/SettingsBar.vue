<template lang="pug">
.SettingsBar
  .SwitchButton(@click='$store.commit("switchPencilMode")' :class='$store.state.game.pencilMode ? "SwitchButton--switched":""')
    .SwitchButton__tooltip Pencil mode {{$store.state.game.pencilMode ? "ON":"OFF"}}
    img(src='../../../images/icons/pencil.svg')
</template>

<script>

export default {
  name: 'SettingsBar'
}
</script>

<style scoped lang="stylus">
.SettingsBar
  bottom -60px
  left 0
  width 100%
  margin-top 15px
  display flex
  font-weight 900
  font-family 'Open Sans'
  justify-content center
  align-items center

.SwitchButton
  display inline-block
  position relative
  margin 5px
  width 42px
  height 42px
  padding 7px 7px
  box-sizing border-box
  border-radius 4px
  font-size 16px
  cursor pointer
  box-sizing border-box
  transition-duration .2s
  box-shadow 0 0 16px #bbb
  z-index 4
  &--switched
    background #0037dd
    img
      filter brightness(150%)
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
    left -45px
    top -33px
    width 140px
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
@media screen and (max-width: 420px)
  .SettingsBar
    display block
    height 60px
    padding 10px 0
</style>
