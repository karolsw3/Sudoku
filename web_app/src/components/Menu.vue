<template lang="pug">
.Menu
  .Menu__slot
    .Menu__logo
    md-menu(md-size="auto" md-align-trigger)
      md-button.md-primary(md-menu-trigger) New game
      md-menu-content
        md-menu-item
          router-link(tag='md-button' to="/play/easy") Easy
        md-menu-item
          router-link(tag='md-button' to="/play/medium") Medium
        md-menu-item
          router-link(tag='md-button' to="/play/hard") Hard
    router-link(tag='md-button' to="/leaderboard") Leaderboard
  .Menu__slot
    router-link.md-primary(tag='md-button' to="/login" v-if='!this.$store.state.userLogged') Login
    router-link(tag='md-button' to="/register" v-if='!this.$store.state.userLogged') Register
    button.Menu__user(v-if='this.$store.state.userLogged') U
</template>

<script>
export default {
  name: 'Menu'
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
    background #0445b7
    border-radius 50%
    border none
    width 40px
    height 40px
    display flex
    align-items center
    justify-content center
    color white
    font-size 1.3em
    font-weight 700
    cursor pointer
    transition-duration .12s
    &:hover
      opacity 0.8
</style>
