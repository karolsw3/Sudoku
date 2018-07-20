<template lang="pug">
.Login
  div(v-if="loading")
    md-progress-bar(md-mode='indeterminate')
    md-progress-bar.md-primary(md-mode='indeterminate')
  ColumnPanel
    Input(placeholder="Username" type="text" ref="username" @valueChanged='checkIfInputsAreFilled')
    Input(placeholder="Password" type="password" ref="password" @valueChanged='checkIfInputsAreFilled')
    Button(@clicked="login" :disabled="!allInputsFilled") Login
  md-snackbar(:md-position='position' :md-active.sync='error' md-persistent='')
    span {{errorMessage}}
    md-button.md-accent(@click='error = false') Close
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'
import axios from 'axios'
import util from '@/util.js'

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
          username: this.$refs.username.value,
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
      axios.post('/api/login', data)
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
    checkIfInputsAreFilled () {
      if (
        this.$refs.username.value !== '' && !this.$refs.username.invalid &&
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
</style>
