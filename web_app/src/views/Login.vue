<template lang="pug">
.Login
  ColumnPanel
    Input(placeholder="Username" type="text")
    Input(placeholder="Password" type="password")
    Button Login
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'

export default {
  name: 'Login',
  components: {
    ColumnPanel, Input, Button
  }
}
</script>

<style scoped lang="stylus">
.Login
  box-sizing border-box
  height 100%
  width 100%
  background white
  display flex
  align-items center
  justify-content center
</style>
