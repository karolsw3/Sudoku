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
  name: 'App',
  components: {
    Menu, Footer
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
  height 100%
  display flex
  flex-direction column
</style>
