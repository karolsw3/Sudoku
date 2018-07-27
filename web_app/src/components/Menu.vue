<template lang="pug">
.Menu
  .Menu__slot
    .Menu__logo
    router-link.md-primary(tag='md-button' to="/") Dashboard
    router-link(tag='md-button' to="/leaderboard") Leaderboard
  .Menu__slot
    router-link.md-primary(tag='md-button' to="/login" v-if='!this.$store.state.user.logged') Login
    router-link(tag='md-button' to="/register" v-if='!this.$store.state.user.logged') Register
    .Menu__user(v-if='this.$store.state.user.logged') Hello, {{this.$store.state.user.login}}!
    md-button(v-if='this.$store.state.user.logged' @click='logout') Logout
</template>

<script>
export default {
  name: 'Menu',
  methods: {
    logout () {
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/v1/auth/logout', true)
      xhr.onload = (response) => {
        switch (response.target.status) {
          case 404:
            this.error = true
            this.errorMessage = '404 - Once upon a time there was a marvelous API. Was.'
            break
          case 204:
            // Success
            this.$store.commit('logout')
            break
        }
      }
      xhr.send('')
    }
  }
}
</script>

<style scoped lang="stylus">
.Menu
  width 100%
  background white
  position relative
  display flex
  align-items center
  justify-content space-between
  box-sizing border-box
  padding 5px 20px
  &__slot
    display flex
    align-items center
    justify-content center
  &__logo
    width 80px
    height 80px
    margin-right 15px
    background-image url(../../../images/logo.png)
    background-size cover
    cursor pointer
  &__user
    border none
    height 40px
    padding 0 10px
    border-radius 2px
    display flex
    align-items center
    justify-content center
    color #222
    cursor default
    background #eee
    transition-duration .12s
</style>
