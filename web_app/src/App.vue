<template lang="pug">
  #app
    TheNavbar
    router-view
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
  min-height 100vh
a
  text-decoration none
  color inherit
  margin 0
  padding 0
#app
  font-family 'Avenir', Helvetica, Arial, sans-serif
  -webkit-font-smoothing antialiased
  -moz-osx-font-smoothing grayscale
  text-align center
  color #2c3e50
  min-height 100vh
  display flex
  color #222
  background-color white
  background-image url('../../images_compressed/background.svg')
  flex-direction column
  overflow-x hidden
  background-position bottom right
  background-repeat no-repeat
  background-size 1950px
</style>
