<template lang="pug">
.NumberSelector
  .NumberSelector__slot(v-for="n in 9" @click="$emit('numberSelected', n)")
    NumberButton {{n}}
  .NumberSelector__slot(@click="$emit('numberSelected', 0)")
    NumberButton(isPrimary="true") 0
</template>

<script>
import NumberButton from '@/components/NumberButton.vue'

export default {
  name: 'NumberSelector',
  components: { NumberButton }
}
</script>

<style scoped lang="stylus">
.NumberSelector
  bottom -60px
  left 0
  width 100%
  margin-top 15px
  display flex
  font-weight 900
  font-family 'Open Sans'
  justify-content center
  align-items center
  &__slot
    width auto
@media screen and (max-width: 420px)
  .NumberSelector
    display block
    height 60px
    padding 10px 0
    &__slot
      display inline-block
</style>
