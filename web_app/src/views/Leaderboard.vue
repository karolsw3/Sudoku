<template lang="pug">
  .leaderboard
    ProgressSpinner(:loading='loading')
    md-table( md-sort='position' md-sort-order='asc' md-card)
      md-table-toolbar
        h1.md-title Leaders
      md-table-row(slot='md-table-row')
        md-table-cell(md-label='Position') Position
        md-table-cell(md-label='Username') Username
        md-table-cell(md-label='Points') Points
        md-table-cell(md-label='Games played') Games played
      md-table-row(slot='md-table-row' v-for='(leader, index) in leaders' :key='index')
        md-table-cell {{ index }}
        md-table-cell {{ leader.username }}
        md-table-cell {{ leader.points_total }}
        md-table-cell {{ leader.games_total }}
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
import ProgressSpinner from '@/components/ProgressSpinner.vue'
import axios from 'axios'

export default {
  name: 'leaderboard',
  components: { NumberButton, ProgressSpinner },
  created () {
    this.loading = true
    axios.get('/api/v1/check/leaderboard?of=users')
      .then((response) => {
        this.loading = false
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
      leaders: [],
      loading: false
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
