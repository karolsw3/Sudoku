<template lang="pug">
.Register
  ColumnPanel
    Input(placeholder="Username" type="text")
    Input(placeholder="Email" type="email")
    Input(placeholder="Password" type="password" id="register__password")
    Input(placeholder="Repeat password" type="password" id="register__confirm_password")
    p TODO: I'm not a robot
    Button Register
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'

export default {
  name: 'Register',
  components: {
    ColumnPanel, Input, Button
  }
}
</script>

<style scoped lang="stylus">
.Register
  box-sizing border-box
  height 100%
  width 100%
  background white
  display flex
  align-items center
  justify-content center
</style>
