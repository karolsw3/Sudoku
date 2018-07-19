<template lang="pug">
  .leaderboard
    table.leaderboard__table
      tr
        th Position
        th Username
        th Points
        th Games played
      tr(v-for='leader in leaders')
        td {{leader.position}}
        td {{leader.username}}
        td {{leader.points}}
        td {{leader.gamesPlayed}}
    .leaderboard__navigation
      .leaderboard__slot(v-for='n in 5' v-if='(page - n) > 0')
        NumberButton {{page - n}}
      .leaderboard__slot
        NumberButton {{page}}
      .leaderboard__slot(v-for='n in 5')
        NumberButton {{page + n}}
      .leaderboard__slot
        p ...
      .leaderboard__slot
        NumberButton {{lastPage}}
</template>

<script>
import NumberButton from '@/components/NumberButton.vue'

export default {
  name: 'leaderboard',
  components: { NumberButton },
  data: function () {
    return {
      page: 0,
      lastPage: 93,
      leaders: [
        {
          position: 1,
          username: 'John Smith',
          points: 10987,
          gamesPlayed: 546
        },
        {
          position: 2,
          username: 'Mark Brown',
          points: 7451,
          gamesPlayed: 394
        }
      ]
    }
  }
}
</script>

<style lang="stylus">
.leaderboard
  width 100%
  height 100%
  display flex
  flex-direction column
  justify-content center
  align-items center
  &__table
    max-width 900px
    border-collapse collapse
    th, td
      padding 7px 23px
      border 1px solid #eee
  &__navigation
    width auto
    height 40px
    margin 20px auto
    display flex
  &__slot
    width auto
</style>
