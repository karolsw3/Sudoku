<template lang="pug">
.Register
  ProgressBar(v-if="loading")
  ColumnPanel
    Input(placeholder="Login" type="text" ref="login" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Email" type="email" ref="email" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Password" type="password" ref="password" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Repeat password" type="password" ref="confirm_password" @valueChanged='validatePasswords(); checkIfInputsAreFilled()')
    MainButton(@clicked="register" :disabled="!allInputsFilled") Register
  md-snackbar(:md-active.sync='error' md-persistent)
    span {{errorMessage}}
    MainButton(@click='error = false') Close
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import base64 from 'base-64'
import util from '@/util.js'
import MainButton from '@/components/MainButton.vue'
import ProgressBar from '@/components/ProgressBar.vue'

export default {
  name: 'Register',
  components: {
    ColumnPanel, Input, MainButton, ProgressBar
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
          username: this.$refs.login.value,
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
      xhr.onload = (response) => {
        this.loading = false
        switch (response.target.status) {
          case 404:
            this.error = true
            this.errorMessage = '404 - Once upon a time there was a marvelous API. Was.'
            break
          case 409:
            // Unauthorized
            this.error = true
            this.errorMessage = '409 - Another user with that login already exists. Maybe you should add a bunch of random numbers at the end?'
            break
          case 201:
            // Success
            let responseData = JSON.parse(response.target.response)
            this.$store.commit('login', {
              login: this.$refs.login.value,
              email: responseData.email,
              pointsTotal: responseData.points_total,
              gamesTotal: responseData.games_total,
              isAdmin: responseData.is_admin
            })
            this.$router.push('/')
            break
        }
      }
      xhr.send('data=' + base64.encode(JSON.stringify(data)))
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
        this.$refs.login.value !== '' && !this.$refs.login.invalid &&
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
  height 2px
</style>
