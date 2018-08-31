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
      .dashboard__cell(:class='$store.state.user.logged ? "dashboard__cell--stats-logged" : "dashboard__cell--stats"')
        .stats(v-if='$store.state.user.logged')
          h1.stats__header Your stats
          .stats__row
            .stats__statName Points
            .stats__statValue {{$store.state.user.pointsTotal}}
          .stats__row
            .stats__statName Games played
            .stats__statValue {{$store.state.user.gamesTotal}}
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
  box-sizing border-box
  background-color white
  background-image url('../../../images/background.svg')
  background-position bottom
  background-repeat repeat-x
  background-size 1300px
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
      height 300px
      background-image url('../../../images/gui/play.svg')
      background-size cover
      grid-area a
    &--leaderboard
      background-image url('../../../images/gui/leaderboard.svg')
      background-size cover
    &--stats
      margin 6px 8px
      line-height 2.3em
      cursor default
      padding 0 20px
      color #fff
      text-shadow 0 0 12px #c20d4f
      text-align left
      font-family 'Open Sans', sans-serif
      background-image url('../../../images/gui/createAccount.svg')
      background-size cover
    &--stats-logged
      cursor default
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
    z-index 999
.stats
  padding 20px 70px
  box-sizing border-box
  &__header
    display block
    text-transform uppercase
    font-weight 900
    color #123ffc
    margin-top 30px
  &__row
    margin 10px 0
    height 20px
    display flex
    justify-content space-around
    align-items center
  &__statName
    width 50%
    text-align left
    color #b3b3b3
    font-size 18px
    text-transform capitalize
  &__statValue
    width 50%
    text-align right
    color black
    font-size 22px
    font-weight 900
</style>
