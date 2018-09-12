<template lang="pug">
.ProgressBar
  md-progress-bar.md-primary(md-mode='indeterminate')
</template>

<script>
export default {
  name: 'ProgressBar'
}
</script>

<style scoped lang="stylus">
.ProgressBar
  position absolute
  top 0
  left 0
  width 100%
  height 2px
  z-index 999
.md-progress-bar
  height 2px !important
</style>
