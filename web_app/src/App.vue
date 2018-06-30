<template>
  <div id="app">
    <Menu id="nav">
      <router-link to="/">Home</router-link>
      <router-link to="/about">About</router-link>
    </Menu>
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
