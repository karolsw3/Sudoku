<template lang="pug">
  #app
    TheNavbar
    router-view.routerView
    TheFooter
</template>

<script>
import TheNavbar from '@/components/TheNavbar.vue'
import TheFooter from '@/components/TheFooter.vue'

export default {
  name: 'Sudoku',
  components: {
    TheNavbar, TheFooter
  }
}
</script>

<style lang="stylus">
@import url('https://fonts.googleapis.com/css?family=Open+Sans:800')

html,body
  margin 0
  padding 0
  font-size 14px
  height 100%
  min-height 100%

a
  text-decoration none
  color inherit
  margin 0
  padding 0

#app
  font-family 'Avenir', Helvetica, Arial, sans-serif
  text-align center
  min-height 100%
  height 100%
  color #222
  background-color white
  background-image url('../../images_compressed/background.svg')
  background-position bottom right
  background-repeat no-repeat
  background-size 1950px
</style>
