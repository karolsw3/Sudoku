<template lang="pug">
  .hello Hi there
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>

<style scoped lang="stylus">
  .hello
    font-size 40px
</style>
