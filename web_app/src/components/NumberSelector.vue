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
  position absolute
  bottom -60px
  left 0
  width 100%
  height 40px
  display flex
  &__slot
    width auto
</style>
