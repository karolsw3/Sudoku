<template lang="pug">
.Login
  div(v-if="loading")
    md-progress-bar(md-mode='indeterminate')
    md-progress-bar.md-primary(md-mode='indeterminate')
  ColumnPanel
    Input(placeholder="Login" type="text" ref="login" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Password" type="password" ref="password" @valueChanged='checkIfInputsAreFilled')
    Button(@clicked="login" :disabled="!allInputsFilled") Login
  md-snackbar(:md-active.sync='error' md-persistent)
    span {{errorMessage}}
    md-button.md-accent(@click='error = false') Close
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'
import util from '@/util.js'
import base64 from 'base-64'

export default {
  name: 'Login',
  components: {
    ColumnPanel, Input, Button
  },
  data: function () {
    return {
      error: false,
      errorMessage: '',
      allInputsFilled: false,
      loading: false
    }
  },
  methods: {
    login: function (event) {
      if (this.allInputsFilled) {
        this.loading = true
        this.error = false
        let data = {
          username: this.$refs.login.value,
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
            this.sendLoginRequest(data)
          }
        )
      }
    },
    sendLoginRequest (data) {
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/v1/auth/login', true)
      xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded')
      xhr.onload = (response) => {
        this.loading = false
        switch (response.target.status) {
          case 404:
            this.error = true
            this.errorMessage = '404 - Once upon a time there was a marvelous API. Was.'
            break
          case 401:
            // Unauthorized
            this.error = true
            this.errorMessage = 'You have entered John Doe\'s password. Maybe your login is "john_doe1997"?'
            break
          case 202:
            // Success
            let responseData = JSON.parse(response.target.response)
            this.$store.commit('login', {
              login: this.$refs.login.value,
              email: responseData.email,
              pointsTotal: responseData.points_total,
              isAdmin: responseData.is_admin
            })
            this.$router.push('/')
            break
        }
      }
      xhr.send('data=' + base64.encode(JSON.stringify(data)))
    },
    checkIfInputsAreFilled () {
      if (
        this.$refs.login.value !== '' && !this.$refs.login.invalid &&
        this.$refs.password.value !== '' && !this.$refs.password.invalid
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
.Login
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
