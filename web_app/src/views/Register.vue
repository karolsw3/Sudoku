<template lang="pug">
.Register
  div(v-if="loading")
    md-progress-bar(md-mode='indeterminate')
    md-progress-bar.md-primary(md-mode='indeterminate')
  ColumnPanel
    Input(placeholder="Username" type="text" ref="username" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Email" type="email" ref="email" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Password" type="password" ref="password" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Repeat password" type="password" ref="confirm_password" @valueChanged='validatePasswords(); checkIfInputsAreFilled()')
    p TODO: I'm not a robot
    Loading(v-if="loading")
    Button(@clicked="register" :disabled="!allInputsFilled") Register
  md-snackbar(:md-position='position' :md-active.sync='error' md-persistent='')
    span {{errorMessage}}
    md-button.md-accent(@click='error = false') Close
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'
import base64 from 'base-64'
import util from '@/util.js'

export default {
  name: 'Register',
  components: {
    ColumnPanel, Input, Button
  },
  data: function () {
    return {
      error: false,
      errorMessage: '',
      loading: false,
      allInputsFilled: false
    }
  },
  methods: {
    register: function (event) {
      if (this.allInputsFilled) {
        this.loading = true
        this.error = false
        let data = {
          login: this.$refs.username.value,
          email: this.$refs.email.value,
          password: this.$refs.password.value
        }
        util.methods.derivePassword(
          data.password,
          (error) => {
            this.error = true
            this.errorMessage = error
          },
          (password) => {
            data.password = password
            this.sendRegisterRequest(data)
          }
        )
      }
    },
    sendRegisterRequest (data) {
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/v1/auth/new', true)
      xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded')
      xhr.onload = function () {
        console.log(this.responseText)
        this.$store.commit('userLogged', true)
        this.loading = false
      }
      xhr.send(base64.encode(JSON.stringify(data)))
    },
    validatePasswords () {
      let confirmPassword = this.$refs.confirm_password
      if (this.$refs.password.value !== confirmPassword.value) {
        confirmPassword.invalid = true
        confirmPassword.errorMessage = 'Passwords doesn\'t match'
      } else {
        confirmPassword.invalid = false
        confirmPassword.errorMessage = ''
      }
    },
    checkIfInputsAreFilled () {
      if (
        this.$refs.username.value !== '' && !this.$refs.username.invalid &&
        this.$refs.password.value !== '' && !this.$refs.password.invalid &&
        this.$refs.email.value !== '' && !this.$refs.email.invalid &&
        this.$refs.confirm_password.value !== '' && !this.$refs.confirm_password.invalid
      ) {
        this.allInputsFilled = true
      } else {
        this.allInputsFilled = false
      }
    }
  }
}
</script>

<style scoped lang="stylus">
.Register
  position relative
  box-sizing border-box
  height 100%
  width 100%
  background white
  display flex
  align-items center
  justify-content center
.md-progress-bar
  position absolute
  top 0
  left 0
  width 100%
</style>
