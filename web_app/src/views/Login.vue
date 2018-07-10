<template lang="pug">
.Login
  ColumnPanel
    Input(placeholder="Username" type="text" id="login__username")
    Input(placeholder="Password" type="password" id="login__password")
    ErrorMessageBox(v-if="error") {{errorMessage}}
    Loading(v-if="loading")
    Button(@clicked="login") Login
</template>``

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import ErrorMessageBox from '@/components/ErrorMessageBox.vue'
import Button from '@/components/Button.vue'
import Loading from '@/components/Loading.vue'
import axios from 'axios'
import scrypt from 'scrypt-js'

export default {
  name: 'Login',
  components: {
    ColumnPanel, Input, Button, ErrorMessageBox, Loading
  },
  data: function () {
    return {
      error: false,
      errorMessage: '',
      loading: false
    }
  },
  methods: {
    login: function (event) {
      this.loading = true
      this.error = false
      let data = {
        username: this.$store.state.login__username,
        password: Buffer.from(this.$store.state.login__password)
      }

      var salt = Buffer.from('Sudoku')

      scrypt(data.password, salt, Math.pow(2, 14), 8, 1, 64, (error, progress, key) => {
        if (error) {
          this.error = true
          this.errorMessage = 'Error with password derivation'
        } else if (key) {
          data.password = ''
          key.map((item) => {
            data.password += item.toString(16)
          })
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
        }
      })
    }
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
