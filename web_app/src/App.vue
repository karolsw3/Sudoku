<template lang="pug">
  #app
    Menu
    .container
      News
      router-view
    Footer
</template>

<script>
import Menu from '@/components/Menu.vue'
import Footer from '@/components/Footer.vue'
import News from '@/components/News.vue'

export default {
  name: 'Sudoku',
  components: {
    Menu, Footer, News
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
  height 100%
  display flex
  color #222
  background-color white
  background-image url('../../images_compressed/background.svg')
  flex-direction column
  overflow-x hidden
  background-position bottom right
  background-repeat no-repeat
  background-size 1950px
.container
  height calc(100% - 62px)
  display flex
</style>
