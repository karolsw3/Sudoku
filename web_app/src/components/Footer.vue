<template lang="pug">
.Footer
  .Footer__logo
  p An open-source app by <b>Galactim</b>
  
</template>

<script>
export default {
  name: 'Footer'
}
</script>

<style scoped lang="stylus">
.Footer
  position absolute
  width 100%
  display flex
  flex-direction column
  padding 5px 0
  align-items center
  justify-content center
  text-align center
  color #cccccc
  bottom 0 !important
  &__logo
    width 32px
    height 30px
    background-image url('../../../images/galactim_logo_mini.png')
    background-size cover
</style>
