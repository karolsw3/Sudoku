<template lang="pug">
  .leaderboard
    ProgressBar(v-if='loading')
    .leaderboard__title Top 10 players
    table
      tr.mainRow
        th Position
        th Username
        th Points
        th Games played
      tr(v-for='(leader, index) in leaders' :key='index')
        td <b>{{ index + 1 }}</b>
        td {{ leader.username }}
        td {{ leader.points_total }}
        td {{ leader.games_total }}
</template>

<script>
import NumberButton from '@/components/NumberButton.vue'
import ProgressBar from '@/components/ProgressBar.vue'
import axios from 'axios'

export default {
  name: 'leaderboard',
  components: {
    NumberButton, ProgressBar
  },
  created () {
    this.loading = true
    axios.get('/api/v1/check/leaderboard?of=users&count=10')
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
  margin 0 auto
  height 100%
  position relative
  display flex
  flex-direction column
  justify-content center
  align-items center
  &__title
    text-align center
    font-size 30px
    font-weight 900
    color #222
    text-transform uppercase
    margin 30px auto
table
  margin 0 auto
  width 95%
  max-width 900px
  border-collapse collapse
  box-shadow 0 0 16px 3px #e4e4e4
  td, th
    padding 10px 6px
    color #222
    font-size 1.2em
  th
    color #123ffc
    font-weight 900
  tr
    background white
    transition-duration .25s
    &:nth-child(even)
      background #f4f4f4
    &:hover
      background #eee
  .mainRow
    background white
    &:hover
      background white
</style>
