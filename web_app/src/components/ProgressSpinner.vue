<template lang="pug">
.ProgressSpinner
  md-progress-spinner(v-if='loading' md-mode="indeterminate")
</template>

<script>
export default {
  name: 'ProgressSpinner',
  props: ['loading']
}
</script>

<style scoped lang="stylus">
.ProgressSpinner
  position fixed
  display inline-flex
  justify-content center
  align-items center
  left 0
  right 0
  z-index 999
  width 100px
  height 100px
  top 50%
  left 50%
  margin-top -50px /* Negative half of height. */
  margin-left -50px /* Negative half of width. */
</style>
