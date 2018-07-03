<template lang="pug">
.ErrorMessageBox
  slot
</template>

<script>
export default {
  name: 'ErrorMessageBox'
}
</script>

<style scoped lang="stylus">
.ErrorMessageBox
  margin-top 15px
  width 250px
  height 30px
  background #ff4455
  color white
  border-radius 2px
  display flex
  align-items center
  justify-content center
  box-shadow inner 0 0 16px #00000066
</style>
