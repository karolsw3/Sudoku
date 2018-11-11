<template lang="pug">
  .leaderboard
    ProgressBar(v-if='loading')
    .leaderboard__title Top 10 {{$route.params.type === "games" ? "games":"players"}}
    table(v-if='$route.params.type === "players"')
      tr.mainRow
        th Position
        th Username
        th Points
        th Games played
      tr(v-for='(leader, index) in results' :key='index')
        td <b>{{ index + 1 }}</b>
        td {{ leader.username }}
        td {{ leader.points_total }}
        td {{ leader.games_total }}
    table(v-if='$route.params.type === "games"')
      tr.mainRow
        th Position
        th Username
        th Difficulty
        th Time
      tr(v-for='(game, index) in results' :key='index')
        td <b>{{ index + 1 }}</b>
        td {{ game.display_name }}
        td {{ game.difficulty === 1 ? "Easy":(game.difficulty === 2 ? "Medium":"Hard") }}
        td {{ Math.floor(game.solution_duration_secs / 60) + "m " + game.solution_duration_secs % 60 + "s"}}
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
    let url = this.$route.params.type === "games" ? '/api/v1/check/leaderboard?count=10':'/api/v1/check/leaderboard?of=users&count=10'
    axios.get(url)
      .then((response) => {
        this.loading = false
        this.results = response.data
      })
      .catch((error) => {
        console.error(error)
      })
  },
  data: function () {
    return {
      page: 0,
      lastPage: 93,
      results: [],
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
  background transparent
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
@media screen and (max-width: 560px)
  .leaderboard
    display inline-block
    padding-top 10px
</style>
