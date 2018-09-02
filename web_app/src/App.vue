<template>
  <div id="app">
    <Menu/>
    <router-view/>
    <Footer/>
  </div>
</template>

<script>
import Menu from '@/components/Menu.vue'
import Footer from '@/components/Footer.vue'

export default {
  name: 'Sudoku',
  components: {
    Menu, Footer
  },
  metaInfo: {
    title: 'The best Sudoku app on the web'
  }
}
</script>

<style lang="stylus">
html,body
  margin 0
  padding 0
  font-size 14px
  height 100vh
a
  text-decoration none
  margin 0
  padding 0
#app
  font-family 'Avenir', Helvetica, Arial, sans-serif
  -webkit-font-smoothing antialiased
  -moz-osx-font-smoothing grayscale
  text-align center
  color #2c3e50
  height 100vh
  display flex
  flex-direction column
  overflow hidden
</style>
