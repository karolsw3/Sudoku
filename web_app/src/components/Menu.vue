<template lang="pug">
.Menu
  .Menu__slot
    .Menu__logo
    router-link(to="/")
      MainButton Dashboard
  .Menu__slot
    router-link(to="/login" v-if='!this.$store.state.user.logged')
      MainButton Login
    router-link(to="/register" v-if='!this.$store.state.user.logged')
      MainButton Register
    .Menu__user(v-if='this.$store.state.user.logged') Hello, {{this.$store.state.user.login}}!
    MainButton(v-if='this.$store.state.user.logged' @click='logout') Logout
</template>

<script>
import MainButton from '@/components/MainButton.vue'

export default {
  name: 'Menu',
  components: { MainButton },
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
  position relative
  width 100%
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
    width 300px
    height 52px
    margin-right 15px
    background-image url(../../../images/logo.svg)
    background-size cover
    cursor pointer
  &__user
    border none
    font-weight 900
    height 40px
    padding 0 20px
    border-radius 2px
    display flex
    align-items center
    justify-content center
    color #333
    cursor default
    background #eee
    transition-duration .12s
</style>
