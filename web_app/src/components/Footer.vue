<template lang="pug">
.Footer
  span An open-source app by <a href='https://github.com/Galactim' target='_blank'><b>Galactim</b></a>
  span |
  span Design by <a href='https://github.com/karolsw3' target='_blank'><b>Karol Świerczek</b></a>
</template>

<script>
export default {
  name: 'Footer'
}
</script>

<style scoped lang="stylus">
.Footer
  position absolute
  background white
  width 100%
  display block
  padding 10px 0
  text-align center
  color #cccccc
  bottom 0 !important
  z-index 3
  span
    display inline-block
    margin 0 5px
  a
    text-decoration none
@media screen and (max-width: 700px)
  .Footer
    display none
</style>
