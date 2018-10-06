<template lang="pug">
.Stats
  .Stats__column
    h1 Your stats
    .Stats__row
      .Stats__statName Points
      .Stats__statValue {{$store.state.user.pointsTotal}}
    h2 Games won
    .Stats__row
      .Stats__statName Easy
      .Stats__statValue {{$store.state.user.gamesTotalEasy}}
    .Stats__row
      .Stats__statName Medium
      .Stats__statValue {{$store.state.user.gamesTotalMedium}}
    .Stats__row
      .Stats__statName Hard
      .Stats__statValue {{$store.state.user.gamesTotalHard}}
  .Stats__column
    h1 Badges
    .Stats__badges
      .Stats__badge
      .Stats__badge
      .Stats__badge
      .Stats__badge
      .Stats__badge
      .Stats__badge
      .Stats__badge
      .Stats__badge
</template>

<script>
export default {
  name: 'Stats'
}
</script>

<style scoped lang="stylus">
.Stats
  width 100%
  display flex
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
    width 42px
    height 42px
    border-radius 100px
    background #eee
</style>
