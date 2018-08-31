<template lang="pug">
  .leaderboard
    ProgressSpinner(:loading='loading')
    table
      tr
        th Position
        th Username
        th Points
        th Games played
      tr(v-for='(leader, index) in leaders' :key='index')
        td {{ index + 1 }}
        td {{ leader.username }}
        td {{ leader.points_total }}
        td {{ leader.games_total }}
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
  margin 0 auto
  width 100%
  max-width 900px
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
table
  width 100%
  border-collapse collapse
  border 2px solid #eee
  td, th
    border 2px solid #eee
    padding 7px 12px
    color #222
    font-size 1.2em
  th
    color #123ffc
    font-weight 900
</style>
