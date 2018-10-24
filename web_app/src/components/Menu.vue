<template lang="pug">
div
  .Menu
    .Menu__slot
      router-link(tag="div" to="/")
        .Menu__logo
    .Menu__slot
      router-link(tag="div" to="/login" v-if='!this.$store.state.user.logged')
        MainButton Login
      router-link(tag="div" to="/register" v-if='!this.$store.state.user.logged')
        MainButton(primary='true') Register
      .Menu__user(v-if='this.$store.state.user.logged') Hello, {{this.$store.state.user.login}}!
      MainButton(v-if='this.$store.state.user.logged' v-on:clicked='logout') Logout
  .Menu.Menu--mobile
    .Menu__slot(@click='selectedButton = "dashboard"')
      router-link.Menu__icon.iconButton.iconButton--dashboard(to="/" tag='div' :class='selectedButton === "dashboard" ? "iconButton--dashboardActive" : ""')
    .Menu__slot(v-if='!this.$store.state.user.logged')
      div(@click='selectedButton = "login"')
        router-link.Menu__icon.iconButton.iconButton--login(to="/login" tag='div' :class='selectedButton === "login" ? "iconButton--loginActive" : ""')
      div(@click='selectedButton = "register"')
        router-link.Menu__icon.iconButton.iconButton--register(to="/register" tag='div' :class='selectedButton === "register" ? "iconButton--registerActive" : ""')
    .Menu__slot(v-else)
      p Hello, {{this.$store.state.user.login}}!
      .Menu__icon.iconButton.iconButton--logout(@click='logout' tag='div')
</template>

<script>
import MainButton from '@/components/buttons/MainButton.vue'

export default {
  name: 'Menu',
  components: { MainButton },
  data: function () {
    return {
      selectedButton: 'dashboard'
    }
  },
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
  padding 3px 15px
  border-bottom 1px solid #f4f4f4
  &--mobile
    display none
  &__slot
    display flex
    align-items center
    justify-content center
    font-family 'Open Sans', sans-serif
  &__logo
    width 254px
    height 44px
    margin-right 15px
    background-image url(../../../images_compressed/logo.svg)
    background-size cover
    cursor pointer
    transition-duration .15s
    &:hover
      opacity .8
  &__user
    border none
    font-weight 900
    padding 9px 14px
    border-radius 2px
    display flex
    align-items center
    justify-content center
    color #333
    cursor default
    background #eee
    transition-duration .12s
@media screen and (max-width: 490px)
  .Menu
    height 50px
    &__logo
      width 150px
      height 26px
    &__user
      padding 1px
      font-size 14px
</style>
