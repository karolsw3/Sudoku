<template lang="pug">
.News(:class='visible ? "News--visible" : ""')
  .News__container
    #fb-root
    h1 The best sudoku online
    h3
      | Sudoku is one of the most popular combinatorial games. <br>
      | The objective is to fill a 9×9 grid with digits so that each column, each row, and each of the nine 3×3 subgrids that compose the grid contains all of the digits from 1 to 9.
    h3 By using our app you can practive solving sudoku boards and gain points by playing. The best players are listed on the leaderboard. Notice that in order to gain points from solving boards you have to login or create an account.
    h3 Happy playing!
    h1 News
    .fb-page(data-href='https://www.facebook.com/pg/thebestsudoku/', data-tabs='timeline', data-width='500', data-height='350', data-small-header='false', data-adapt-container-width='true', data-hide-cover='false', data-show-facepile='true')
      blockquote.fb-xfbml-parse-ignore(cite='https://www.facebook.com/pg/thebestsudoku/')
    .News__arrow(@click='toggle()')
</template>

<script>
export default {
  name: 'News',
  data: function () {
    return {
      visible: true
    }
  },
  methods: {
    toggle () {
      this.visible = !this.visible
    }
  }
}

let d = document
let s = 'script'
let id = 'facebook-jssdk'
var js, fjs = d.getElementsByTagName(s)[0]
js = d.createElement(s)
js.id = id
js.src = 'https://connect.facebook.net/pl_PL/sdk.js#xfbml=1&version=v3.1'
fjs.parentNode.insertBefore(js, fjs)

</script>

<style scoped lang="stylus">
.News
  min-width 0
  width 0
  position relative
  height 100%
  left -600px
  top 0
  display block
  z-index 3
  background white
  margin 0
  box-shadow 2px 0 14px #eee
  text-align left
  transition-duration 0.4s
  h1
    font-size 35px
    font-weight 900
    font-family 'Open Sans', sans-serif
    text-transform uppercase
    color #eaeaea
    margin 30px 0
  h3
    font-size 17px
    font-weight 100
    line-height 1.35em
    color #778
  &--visible
    left 0
    width 600px
    min-width 600px
  &__container
    min-width 600px
    padding 60px 50px
  &__arrow
    width 33px
    height 32px
    background-color #0037dd
    border none
    border-radius 45px
    display flex
    justify-content center
    align-items center
    padding 0px 5px 2px 4px
    position absolute
    right -16px
    top calc(50% - 16px)
    z-index 3
    cursor pointer
    transition-duration .2s
    &:hover
      opacity .9
</style>
