<template lang="pug">
.TheStats
  .TheStats__column
    h1 Your TheStats
    .TheStats__row
      .TheStats__statName Points
      .TheStats__statValue {{$store.state.user.pointsTotal}}
    h2 Boards solved
    .TheStats__row
      .TheStats__statName Easy
      .TheStats__statValue {{$store.state.user.gamesTotalEasy}}
    .TheStats__row
      .TheStats__statName Medium
      .TheStats__statValue {{$store.state.user.gamesTotalMedium}}
    .TheStats__row
      .TheStats__statName Hard
      .TheStats__statValue {{$store.state.user.gamesTotalHard}}
  .TheStats__column
    h1 Badges
    .TheStats__badges
      .TheStats__badge(:class='{"TheStats__badge--active" : $store.state.user.gamesTotal >= 10}')
        .TheStats__tooltip(v-if='$store.state.user.gamesTotal >= 10') <b>Good start</b>: Solved 10 boards
        .TheStats__tooltip(v-else) <b>Good start</b>: Solve 10 boards ({{$store.state.user.gamesTotal}}/10)
      .TheStats__badge(:class='{"TheStats__badge--active" : $store.state.user.gamesTotal >= 40}')
        .TheStats__tooltip(v-if='$store.state.user.gamesTotal >= 40') <b>Confident player</b>: Solved 40 boards
        .TheStats__tooltip(v-else) <b>Confident player</b>: Solve 40 boards ({{$store.state.user.gamesTotal}}/40)
      .TheStats__badge(:class='{"TheStats__badge--active" : $store.state.user.gamesTotal >= 100}')
        .TheStats__tooltip(v-if='$store.state.user.gamesTotal >= 100') <b>Sudokuphile</b>: Solved 100 boards
        .TheStats__tooltip(v-else) <b>Sudokuphile</b>: Solve 100 boards ({{$store.state.user.gamesTotal}}/100)
      .TheStats__badge(:class='{"TheStats__badge--active" : $store.state.user.gamesTotalHard >= 3}')
        .TheStats__tooltip(v-if='$store.state.user.gamesTotalHard >= 3') <b>Intellectual</b>: 3 hard boards solved
        .TheStats__tooltip(v-else) <b>Intellectual</b>: Solve 3 hard boards ({{$store.state.user.gamesTotalHard}}/3)
      .TheStats__badge
      .TheStats__badge
      .TheStats__badge
      .TheStats__badge
</template>

<script>
export default {
  name: 'TheStats'
}
</script>

<style scoped lang="stylus">
.TheStats
  width 100%
  display flex
  background white
  box-sizing border-box
  font-family 'Open Sans', sans-serif
  padding 15px
  h1, h2
    display block
    text-transform uppercase
    font-weight 900
    color #123ffc
    font-size 25px
    text-align left
    margin-top 12px
    white-space pre
  h2
    color #222
    font-size 15px
    margin 12px 0
  &__column
    width 50%
    height 100%
    box-sizing border-box
    &:nth-child(1)
      padding 0 18px 0 0
      border-right 1px solid #eee
    &:nth-child(2)
      padding 0 0 0 18px
  &__row
    margin 3px 0
    height 20px
    display flex
    justify-content space-around
    align-items center
  &__statName
    width 50%
    text-align left
    margin-left 5px
    color #b3b3b3
    font-size 14px
    font-weight 100
    text-transform capitalize
    font-family 'Arial', sans-serif
    white-space pre
  &__statValue
    width 50%
    text-align right
    color #222
    font-size 15px
    font-weight 900
    white-space pre
  &__badges
    display grid
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    grid-gap 5px
    width 136px
  &__badge
    position relative
    width 42px
    height 42px
    border-radius 100px
    background-color #eee
    background-size cover
    cursor pointer
    filter grayscale(100%)
    transition-duration 0.2s
    &:nth-child(1)
      background-image url('../../../images/badges/numbers1.svg')
      border 2px solid #eee
    &:nth-child(2)
      background-image url('../../../images/badges/numbers2.svg')
      border 2px solid #0037dd
    &:nth-child(3)
      background-image url('../../../images/badges/numbers3.svg')
      border 2px solid #ff1166
    &:nth-child(4)
      background-image url('../../../images/badges/hard1.svg')
      border 2px solid #eee
    &--active
      filter none
    &:hover
      .TheStats__tooltip
        display block
  &__tooltip
    opacity 0.9
    display none
    position absolute
    left -99px
    top -33px
    width 240px
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
