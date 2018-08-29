<template lang="pug">
  .leaderboard
    md-table(v-for='(leader, index) in leaders', md-sort='position', md-sort-order='asc', md-card)
      md-table-toolbar
        h1.md-title Leaders
      md-table-row(slot='md-table-row')
        md-table-cell(md-label='Position' md-numeric) {{ index }}
        md-table-cell(md-label='Username' md-sort-by='username') {{ leader.username }}
        md-table-cell(md-label='Points' md-sort-by='points') {{ leader.points_total }}
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
import axios from 'axios'

export default {
  name: 'leaderboard',
  components: { NumberButton },
  created () {
    axios.get('/api/v1/check/leaderboard?of=users')
      .then((response) => {
        this.leaders = response.data
      })
      .catch((error) => {
        console.error(error)
      })
  },
  data: function () {
    return {
      page: 0,
      lastPage: 93,
      leaders: []
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
