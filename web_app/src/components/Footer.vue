<template lang="pug">
.Footer
  p An app by <b>Galactim</b>
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
  z-index 0
@media screen and (max-width: 700px)
  .Footer
    display none
</style>
