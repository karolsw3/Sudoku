<template lang="pug">
  .home
    .dashboard
      md-button.dashboard__cell(@click="selectorOpen = !selectorOpen")
        .dashboard__button.dashboard__button--play Play
      .dashboard__difficultySelector(v-if="selectorOpen")
        router-link(tag='md-button' to="/play/easy") Easy
        router-link(tag='md-button' to="/play/medium") Medium
        router-link(tag='md-button' to="/play/hard") Hard
      router-link(tag='md-button' to="/leaderboard").dashboard__cell.dashboard__cell--leaderboard
        .dashboard__button Leaderboard
      .dashboard__cell.dashboard__cell--stats
        template(v-if='$store.state.user.logged')
          h1 Your stats
          p Points: {{$store.state.user.pointsTotal}}
        template(v-else)
          h1 Create an account to get points from solving boards.
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
      background-image url('../../../images/gui/play.svg')
      background-size cover
      grid-area a
    &--leaderboard
      background-image url('../../../images/gui/leaderboard.svg')
      background-size cover
    &--stats
      margin 6px 8px
      line-height 2.3em
      padding 0 12px
      color #fff
      text-shadow 0 0 12px #c20d4f
      text-align left
      font-family 'Open Sans', sans-serif
      background-image url('../../../images/gui/stats.svg')
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
    background #ff1167
    padding 37px
    color #fff
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
    z-index 2
.stats
  padding 20px
  cursor default
  text-align left
  font-family 'Open Sans', sans-serif
  h1
    font-size 2em
    line-height 1.15
    color #ccc
  p
    font-size 1.5em
    color #444
</style>