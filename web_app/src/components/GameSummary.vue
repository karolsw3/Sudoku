<template lang="pug">
.GameSummary
  .GameSummary__image
  .GameSummary__stats
    h1.GameSummary__header Magnificent!
    h2.GameSummary__subtitle Game summary
    .GameSummary__row
      .GameSummary__statName Time
      .GameSummary__statValue
        span(v-if='solutionDurationHours > 0') {{solutionDurationHours}}h
        span(v-if='solutionDurationMinutes > 0') {{solutionDurationMinutes}}min 
        span {{solutionDuration % 60}}s
    .GameSummary__row
      .GameSummary__statName Difficulty
      .GameSummary__statValue {{difficultyName}}
    .GameSummary__row
      .GameSummary__statName Points gained
      .GameSummary__statValue.GameSummary__statValue--bold {{score}}
    md-button.md-raised.md-primary(v-on:click='$emit("summaryClosed")') Close
</template>

<script>
export default {
  name: 'GameSummary',
  props: ['solutionDuration', 'difficulty', 'score'],
  computed: {
    difficultyName: function () {
      switch (this.difficulty) {
        case 1:
          return 'Easy'
        case 2:
          return 'Medium'
        case 3:
          return 'Hard'
        default:
          return 'Easy'
      }
    },
    solutionDurationHours: function () {
      return Math.floor(this.solutionDuration / (60 * 60))
    },
    solutionDurationMinutes: function () {
      return Math.floor(this.solutionDuration / 60)
    }
  }
}
</script>

<style scoped lang="stylus">
.GameSummary
  position fixed
  left 0
  right 0
  margin 0 auto
  width 481px
  background white
  border-radius 5px
  z-index 4999
  box-shadow 0 0 22px #aaa
  &__image
    width 481px
    height 250px
    background url('../../../images/gui/gameSummary.svg')
    background-size cover
  &__stats
    padding 10px 70px
    box-sizing border-box
  &__header
    display block
    font-size 40px
    text-transform uppercase
    font-weight 900
    color #00115b
    margin-top 30px
  &__subtitle
    color #808080
    font-size 18px
    text-align left
    font-weight 400
    text-transform uppercase
    margin-bottom 25px
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
    font-size 15px
  &__statValue
    width 50%
    text-align right
    color black
    font-size 19px
    &--bold
      font-weight 900
      color #00115b
</style>
