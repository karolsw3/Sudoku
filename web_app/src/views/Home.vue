<template lang="pug">
  .home
    .dashboard  
      md-button.dashboard__cell
        .dashboard__button Play
      md-button.dashboard__cell
      md-button.dashboard__cell
</template>

<script>
export default {
  name: 'home'
}
</script>

<style scoped lang="stylus">
@import url('https://fonts.googleapis.com/css?family=Open+Sans:800')

.home
  width 100%
  height 100%
  display flex
  align-items center
  justify-content center

.dashboard
  margin 0 auto
  width 90%
  max-width 800px
  display grid
  grid-gap 20px
  grid-template 1fr 1fr / 1fr 1fr
  grid-template-areas "a a" "b c"
  &__cell
    position relative
    height 220px
    border-radius 5px
    background #e4e4e4
    cursor pointer
    &:nth-child(1)
      height 369px
      background-image url('../../../images/gui/play.png')
      grid-area a
  &__button
    position absolute
    display flex
    align-items center
    justify-content center
    left -100px
    right 0
    top 110px
    width 200px
    height 50px
    border-radius 50px
    color #222
    font-family 'Open Sans', sans-serif
    text-transform uppercase
    font-weight 900
    font-size 50px
</style>