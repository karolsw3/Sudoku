<template lang="pug">
  .home
    .dashboard
      md-button.dashboard__cell(@click="selectorOpen = !selectorOpen")
        .dashboard__button.dashboard__button--play Play
      .dashboard__difficultySelector(v-if="selectorOpen")
        router-link(tag='md-button' to="/play/easy") Easy
        router-link(tag='md-button' to="/play/medium") Medium
        router-link(tag='md-button' to="/play/hard") Hard
      router-link(tag='md-button' to="/leaderboard").dashboard__cell
        .dashboard__button Leaderboard
      md-button.dashboard__cell
</template>

<script>
export default {
  name: 'home',
  data: function () {
    return {
      selectorOpen: false
    }
  }
}
</script>

<style scoped lang="stylus">
@import url('https://fonts.googleapis.com/css?family=Open+Sans:800')

.md-button + .md-button
    margin 6px 8px

.home
  width 100%
  height 100%
  display flex
  align-items center
  justify-content center

.dashboard
  position relative
  margin 0 auto
  width 90%
  max-width 800px
  display grid
  grid-gap 20px
  grid-template 1fr 1fr/ 1fr 1fr
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
    &:nth-child(2)
      background-image url('../../../images/gui/leaderboard.svg')
      background-size cover
  &__button
    display flex
    align-items center
    justify-content center
    height 50px
    border-radius 50px
    font-family 'Open Sans', sans-serif
    text-transform uppercase
    font-weight 900
    font-size 30px
    background linear-gradient(to right, #ff1167, #f00)
    padding 37px
    color #fff
    box-shadow 0 0 34px #000000b3
    &--play
      font-size 50px
  &__difficultySelector
    grid-area selector
    position absolute
    margin 0 auto
    top 237px
    left 0
    right 0
    height 50px
    width 350px
    border-radius 5px
    background white
    display flex
    justify-content space-around
    align-items center
    transition-duration .2s
    box-shadow 0 0 20px rgba(0, 0, 0, .5)
    z-index 999
</style>