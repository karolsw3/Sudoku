<template lang="pug">
.NumberSelector
  .NumberSelector__slot(v-for="n in 9" @click="$emit('numberSelected', n)") {{n}}
</template>

<script>
export default {
  name: 'NumberSelector'
}
</script>

<style scoped lang="stylus">
.NumberSelector
  position absolute
  bottom -50px
  left 0
  width 100%
  height 40px
  display flex
  border 1px solid #eee
  &__slot
    width 100%
    height 100%
    background white
    color #d9d9d9
    font-size 20px
    display flex
    justify-content center
    align-items center
    border 1px solid #eee
    cursor pointer
    box-sizing border-box
    transition-duration .12s
    &:hover
      background #0445b7
      color white
</style>
