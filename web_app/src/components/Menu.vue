<template lang="pug">
.Menu
  .Menu__slot
    .Menu__logo
    .Menu__button.Menu__button--dropdownBase New game
      .Menu__dropdown
        button.Menu__button.Menu__button--dropdown Easy
        button.Menu__button.Menu__button--dropdown Medium
        button.Menu__button.Menu__button--dropdown Hard
    button.Menu__button Leaderboard
  .Menu__slot
    button.Menu__button Login
    button.Menu__button Register
    button.Menu__user U
</template>

<script>
export default {
  name: 'Menu'
}
</script>

<style scoped lang="stylus">
.Menu
  width 100%
  height 80px
  background white
  position relative
  display flex
  align-items center
  justify-content space-between
  box-sizing border-box
  padding 0 20px
  &:before
    content ''
    position absolute
    height 2px
    width 100%
    bottom -2px
    left 0
    background-image linear-gradient(to right, #001672, #0445b7)
  &__slot
    display flex
    align-items center
    justify-content center
  &__logo
    width 60px
    height 60px
    margin-right 15px
    background-image url(../../../images/logo.png)
    background-size cover
    cursor pointer
  &__button
    width 130px
    height 40px
    padding 10px 20px
    margin-right 15px
    box-sizing border-box
    color #001672
    font-size 1em
    background #eee
    border-radius 2px
    border none
    cursor pointer
    transition-duration .12s
    position relative
    &:hover
      opacity 0.8
    &--dropdownBase
      border-radius-bottom-left 0
      border-radius-bottom-right 0
      &:hover
        opacity 1
        .Menu__dropdown
          display block
    &--dropdown
      background #f4f4f4
      border-radius none
      color #787878
      z-index 2
      &:hover
        color white
        background #a4a4a4
        opacity 0.8
  &__dropdown
    position absolute
    top 100%
    left 0
    width 100%
    background white
    border-radius-bottom-left 2px
    border-radius-bottom-right 2px
    z-index 3
    display none
  &__user
    background #0445b7
    border-radius 50%
    border none
    width 40px
    height 40px
    display flex
    align-items center
    justify-content center
    color white
    font-size 1.3em
    font-weight 700
    cursor pointer
    transition-duration .12s
    &:hover
      opacity 0.8
</style>
