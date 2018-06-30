<template lang="pug">
input.Input(:placeholder="placeholder" :type="type")
</template>

<script>
export default {
  name: 'Input',
  props: ['placeholder', 'type']
}
</script>

<style scoped lang="stylus">
.Input
  width 100%
  max-width 440px
  box-sizing border-box
  padding 19px 20px
  margin-top 30px
  background #f7f7f7
  color #aaa
  border none
  border-radius 2px
</style>
