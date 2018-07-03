<template lang="pug">
.ColumnPanel
  slot
</template>

<script>
export default {
  name: 'ColumnPanel'
}
</script>

<style scoped lang="stylus">
.ColumnPanel
  width 100%
  max-width 500px
  background #fff
  border-radius 2px
  padding 20px
  display flex
  flex-direction column
  align-items center
  justify-content center
</style>
