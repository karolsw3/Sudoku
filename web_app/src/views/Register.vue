<template lang="pug">
.Register
  ColumnPanel
    Input(placeholder="Username" type="text" ref="username" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Email" type="email" ref="email" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Password" type="password" ref="password" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Repeat password" type="password" ref="confirm_password" @valueChanged='validatePasswords(); checkIfInputsAreFilled()')
    p TODO: I'm not a robot
    ErrorMessageBox(v-if="error") {{errorMessage}}
    Loading(v-if="loading")
    Button(@clicked="register" :class="{'Button--disabled' : !allInputsFilled}") Register
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import ErrorMessageBox from '@/components/ErrorMessageBox.vue'
import Button from '@/components/Button.vue'
import Loading from '@/components/Loading.vue'
import axios from 'axios'
import scrypt from 'scrypt-js'
import util from '@/util.js'

export default {
  name: 'Register',
  components: {
    ColumnPanel, Input, Button, ErrorMessageBox, Loading
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
          username: this.$refs.username.value,
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
      axios.post('/api/register', data)
        .then((response) => {
          this.$store.commit('userLogged', true)
          this.loading = false
        })
        .catch((error) => {
          switch (error.response.status) {
            case 404:
              this.error = true
              this.errorMessage = 'Error 404'
              break
            default:
              this.error = true
              this.errorMessage = 'Internal server error'
          }
          this.loading = false
        })
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
  box-sizing border-box
  height 100%
  width 100%
  background white
  display flex
  align-items center
  justify-content center
</style>
