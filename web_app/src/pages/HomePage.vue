<template lang="pug">
  .HomePage
    .dashboard
      .dashboard__cell.dashboard__cell--button(@click="difficultySelectorOpen = !difficultySelectorOpen")
        .dashboard__button New game
        .dashboard__selector(v-if="difficultySelectorOpen")
          router-link.dashboard__selectorButton(tag='div' to="/play/easy")
            .dashboard__selectorIcon.dashboard__selectorIcon--easy
            span
              p Easy
          router-link.dashboard__selectorButton(tag='div' to="/play/medium")
            .dashboard__selectorIcon.dashboard__selectorIcon--medium
            span
              p Medium
          router-link.dashboard__selectorButton(tag='div' to="/play/hard")
            .dashboard__selectorIcon.dashboard__selectorIcon--hard
            span
              p Hard
      .dashboard__cell.dashboard__cell--button.dashboard__cell--leaderboard(@click="leaderboardSelectorOpen = !leaderboardSelectorOpen")
        .dashboard__button Leaderboard
        .dashboard__selector.dashboard__selector--small(v-if="leaderboardSelectorOpen")
          router-link.dashboard__selectorButton(tag='div' to="/leaderboard/players")
            .dashboard__selectorIcon.dashboard__selectorIcon--players
            span
              p Players
          router-link.dashboard__selectorButton(tag='div' to="/leaderboard/games")
            .dashboard__selectorIcon.dashboard__selectorIcon--games
            span
              p Games
      .dashboard__cell(:class='$store.state.user.logged ? "dashboard__cell--stats-logged" : "dashboard__cell--stats"')
        TheStats(v-if='$store.state.user.logged')
        router-link(v-else to="/register" tag="div")
          h1 Create an account to get points and compete with others!
</template>

<script>
import TheStats from '@/components/TheStats.vue'

export default {
  name: 'HomePage',
  components: { TheStats },
  data: function () {
    return {
      difficultySelectorOpen: false,
      leaderboardSelectorOpen: false
    }
  },
  created () {
    window.onbeforeunload = null
  }
}
</script>

<style scoped lang="stylus">

.md-button + .md-button
  margin 6px 8px
.HomePage
  height 100%
  width 100%
  display flex
  flex-direction column
  align-items center
  justify-content center
  box-sizing border-box
  user-select none
  background transparent
.dashboard
  position relative
  margin 0 auto
  max-width 800px
  display grid
  grid-gap 20px
  grid-template 1fr 1fr / 1fr 1fr
  grid-template-areas "a a" "b c"
  &__cell
    position relative
    height 220px
    border-radius 6px
    background #e4e4e4
    cursor pointer
    display flex
    align-items center
    justify-content center
    box-shadow 0 0 3px #00000022
    &:nth-child(1)
      height 229px
      background-image url('../../../images_compressed/gui/play.svg')
      background-size 104%
      grid-area a
    &--button
      transition-duration .4s
      &:hover
        opacity .9
    &--leaderboard
      margin 3px 3px 6px 0
      background-image url('../../../images_compressed/gui/leaderboard.svg')
      background-size 104%
    &--stats
      margin 3px 3px 6px 0
      width 372px
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
      width 372px
      margin 3px 3px 6px 0
      cursor default
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
  &__selector
    grid-area selector
    position absolute
    margin 0 auto
    bottom 11px
    left 0
    right 0
    height 50px
    width 380px
    border-radius 5px
    display flex
    justify-content space-around
    align-items center
    transition-duration .2s
    z-index 2
    &--small
      width 275px
  &__selectorButton
    background #fff
    height 50px
    min-width 112px
    padding 12px 15px 12px 8px
    border-radius 50px
    margin-bottom 0
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
    box-shadow 0 0 3px #00000033
    &:hover
      margin-bottom 8px
  &__selectorIcon
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
    &--players
      background-image url('../../../images/icons/players.svg')
    &--games
      background-image url('../../../images/icons/games.svg')
@media screen and (max-width: 800px)
  .md-button + .md-button
      margin 6px 0
  .home
    margin-top 20px
  .dashboard
    display block
    height 100%
    width 95%
    &__cell
      width 100%
      height 205px
      display inline-flex
      justify-content center
      align-items center
      margin 10px auto
      box-shadow 0 0 16px 3px #e4e4e4
      background-size cover
      z-index 1
      &--stats, &--stats-logged, &--leaderboard, &:nth-child(1)
        margin 12px auto
        height 190px
        text-align center
      &--stats
        background-size auto
    &__selector
      width 310px
      &--small
        width 235px
    &__selectorButton
      height 43px
      font-size 11px
      min-width 90px
    &__selectorIcon
      width 29px
      height 29px
    &__button
      font-size 24px
  .stats
    width 90%
    padding 20px 30px
    &__header
      margin-top 0
      font-size 24px
</style>
