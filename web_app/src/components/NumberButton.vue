<template lang="pug">
md-button.NumberButton(:class='{"md-accent" : isPrimary}')
  slot
</template>

<script>
export default {
  name: 'NumberButton',
  props: ['isPrimary']
}
</script>

<style scoped lang="stylus">
.NumberButton
  min-width 0
  width 34px
  height 34px
  margin 0 3px
  font-size 16px
</style>
