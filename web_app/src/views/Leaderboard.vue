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
      a(v-for='n in 5' v-if='(page - n) > 0')
        .leaderboard__button {{page - n}}
      a
        .leaderboard__button {{page}}
      a(v-for='n in 5')
        .leaderboard__button {{page + n}}
      a
        .leaderboard__button.leaderboard__button--last ... {{lastPage}}
</template>

<script>

export default {
  name: 'leaderboard',
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
    border 2px solid #eee
    th, td
      padding 7px 23px
      border 2px solid #eee
  &__navigation
    width auto
    margin 20px auto
    display flex
  &__button
    margin 0 5px
    padding 8px
    border 1px solid #ccc
    border-radius 5px
    cursor pointer
    &--last
      margin-left 15px
</style>
