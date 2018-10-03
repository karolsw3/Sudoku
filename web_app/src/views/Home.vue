<template lang="pug">
  .home
    .dashboard
      .dashboard__cell.dashboard__cell--button(@click="selectorOpen = !selectorOpen")
        .dashboard__button New game
      .dashboard__difficultySelector(v-if="selectorOpen")
        router-link.dashboard__difficultySelectorButton(tag='div' to="/play/easy")
          .dashboard__difficultySelectorIcon.dashboard__difficultySelectorIcon--easy
          span
            p Easy
        router-link.dashboard__difficultySelectorButton(tag='div' to="/play/medium")
          .dashboard__difficultySelectorIcon.dashboard__difficultySelectorIcon--medium
          span
            p Medium
        router-link.dashboard__difficultySelectorButton(tag='div' to="/play/hard")
          .dashboard__difficultySelectorIcon.dashboard__difficultySelectorIcon--hard
          span
            p Hard
      router-link(tag='p' to="/leaderboard").dashboard__cell.dashboard__cell--button.dashboard__cell--leaderboard
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
          h1 Create an account to get points and compete with others!
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

.md-button + .md-button
    margin 6px 8px
.home
  width auto
  margin 0 auto
  height 100%
  display flex
  align-items center
  justify-content center
  box-sizing border-box
.dashboard
  position relative
  margin 0 auto
  width 100%
  max-width 800px
  display grid
  grid-gap 20px
  grid-template 1fr 1fr/ 1fr 1fr
  grid-template-areas "a a" "b c"
  &__cell
    position relative
    height 220px
    border-radius 10px
    background #e4e4e4
    background-position center
    cursor pointer
    box-shadow 0 0 16px 3px #e4e4e4
    display flex
    align-items center
    justify-content center
    &:nth-child(1)
      height 300px
      background-image url('../../../images_compressed/gui/play.svg')
      background-size 104%
      grid-area a
    &--button
      transition-duration .4s
      &:hover
        box-shadow 0 0 22px 1px #9a9a9a
        background-size 109% !important
    &--leaderboard
      margin 6px 8px
      background-image url('../../../images_compressed/gui/leaderboard.svg')
      background-size 104%
    &--stats
      margin 6px 8px
      line-height 2.3em
      cursor default
      padding 0 20px
      color #fff
      text-shadow 0 0 12px #c20d4f
      text-align left
      font-family 'Open Sans', sans-serif
      background-image url('../../../images_compressed/gui/createAccount.svg')
      background-size cover
      align-items flex-start
    &--stats-logged
      margin 6px 8px
      cursor default
      background-image url('../../../images_compressed/gui/stats.svg')
      background-size cover
      align-items flex-start
  &__button
    display flex
    align-items center
    justify-content center
    height 50px
    border-radius 50px
    font-family 'Open Sans', sans-serif
    text-transform uppercase
    font-weight 900
    font-size 32px
    background #ff1167
    padding 37px
    text-decoration none
    color #fff
  &__difficultySelector
    grid-area selector
    position absolute
    margin 0 auto
    top 237px
    left 0
    right 0
    height 50px
    width 380px
    border-radius 5px
    display flex
    justify-content space-around
    align-items center
    transition-duration .2s
    z-index 999
  &__difficultySelectorButton
    background #eee
    height 50px
    min-width 112px
    padding 12px 15px 12px 8px
    border-radius 50px
    cursor pointer
    box-sizing border-box
    display flex
    justify-content space-between
    align-items center
    font-weight 900
    color #222
    font-family 'Open Sans', sans-serif
    text-transform uppercase
    text-decoration none
    transition-duration .2s
    box-shadow 0 0 20px rgba(0, 0, 0, .5)
    &:hover
      background #ddd
  &__difficultySelectorIcon
    width 38px
    height 38px
    border-radius 32px
    margin-right 10px
    background-color white
    background-size cover
    display inline-block
    &--easy
      background-image url('../../../images/icons/easy.svg')
    &--medium
      background-image url('../../../images/icons/medium.svg')
    &--hard
      background-image url('../../../images/icons/hard.svg')
.stats
  padding 20px 60px
  box-sizing border-box
  font-family 'Open Sans', sans-serif
  &__header
    display block
    text-transform uppercase
    font-weight 900
    color #123ffc
    font-size 32px
    margin-top 30px
    white-space pre
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
    white-space pre
  &__statValue
    width 50%
    text-align right
    color black
    font-size 22px
    font-weight 900
    white-space pre

@media screen and (max-width: 800px)
  .md-button + .md-button
      margin 6px 0
  .home
    min-height 100%
    margin-bottom 25px
  .dashboard
    display block
    height 100%
    width 95%
    &__cell
      width 100%
      height 190px
      display inline-flex
      justify-content center
      align-items center
      margin 10px auto
      z-index 1
      &--stats, &--stats-logged, &--leaderboard, &:nth-child(1)
        margin 12px auto
        height 190px
        text-align center
      &--stats
        background-size auto
    &__difficultySelector
      top 170px
    &__button
      font-size 24px
  .stats
    width 90%
    padding 20px 30px
    &__header
      margin-top 0
      font-size 24px
</style>
