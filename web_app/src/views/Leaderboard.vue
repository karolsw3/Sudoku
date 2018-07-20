<template lang="pug">
  .leaderboard
    md-table(v-model='leaders', md-sort='position', md-sort-order='asc', md-card)
      md-table-toolbar
        h1.md-title Leaders
      md-table-row(slot='md-table-row' slot-scope='{ item }')
        md-table-cell(md-label='Position' md-numeric) {{ item.position }}
        md-table-cell(md-label='Username' md-sort-by='username') {{ item.username }}
        md-table-cell(md-label='Points' md-sort-by='points') {{ item.points }}
        md-table-cell(md-label='Games played' md-sort-by='gamesPlayed') {{ item.gamesPlayed }}
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
  &__navigation
    width auto
    height 40px
    margin 20px auto
    display flex
  &__slot
    width auto
</style>
