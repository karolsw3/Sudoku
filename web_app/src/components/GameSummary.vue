<template lang="pug">
.GameSummary
  .GameSummary__image
    CloseButton(v-on:clicked='$emit("summaryClosed")')
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
    MainButton(v-on:clicked='$emit("reload"); $emit("summaryClosed")') New Game
</template>

<script>
import MainButton from '@/components/buttons/MainButton.vue'
import CloseButton from '@/components/buttons/CloseButton.vue'

export default {
  name: 'GameSummary',
  props: ['solutionDuration', 'difficulty', 'score'],
  components: { MainButton, CloseButton },
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
      return Math.floor((this.solutionDuration / 60) % 60)
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
  width 306px
  background white
  border-radius 10px
  z-index 4999
  box-shadow 0 0 22px #aaa
  &__image
    height 177px
    background url('../../../images/gui/gameSummary.svg')
    border-radius 10px 10px 0 0
    background-size cover
  &__stats
    padding 20px 25px
    box-sizing border-box
  &__header
    display block
    text-transform uppercase
    font-weight 900
    color #0037dd
  &__subtitle
    color #808080
    font-size 16px
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
    font-size 13px
  &__statValue
    width 50%
    text-align right
    color black
    font-size 15px
    span
      margin 0 5px
    &--bold
      font-weight 900
      color #0037dd
</style>
