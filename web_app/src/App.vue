<template>
  <div id="app">
    <Menu/>
    <div class="container">
      <News/>
      <router-view/>
    </div>
    <Footer/>
  </div>
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
  color #222
  background-color white
  background-image url('../../images_compressed/background.svg')
  background-position bottom right
  background-repeat no-repeat
  background-size 1950px
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
  flex-direction column
  overflow-x hidden
.container
  height calc(100% - 62px)
  display flex
</style>
