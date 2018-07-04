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
  top 150px
  left 10px
  width 150px
  height 150px
  display grid
  grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
  border 1px solid #eee
  &__slot
    width 100%
    height 100%
    background white
    color #d9d9d9
    font-size 25px
    display flex
    justify-content center
    align-items center
    border 1px solid #eee
    cursor pointer
    box-sizing border-box
    transition-duration .12s
    &:hover
      border 2px solid #0445b7
      color #0445b7
</style>
