<template lang="pug">
.Register
  ColumnPanel
    Input(placeholder="Username" type="text" id="register__username")
    Input(placeholder="Email" type="email" id="register__email")
    Input(placeholder="Password" type="password" id="register__password")
    Input(placeholder="Repeat password" type="password" id="register__confirm_password")
    p TODO: I'm not a robot
    ErrorMessageBox(v-if="error") {{errorMessage}}
    Loading(v-if="loading")
    Button(@clicked="register") Register
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import ErrorMessageBox from '@/components/ErrorMessageBox.vue'
import Button from '@/components/Button.vue'
import Loading from '@/components/Loading.vue'
import axios from 'axios'
import scrypt from 'scrypt-js'

export default {
  name: 'Register',
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
    register: function (event) {
      this.loading = true
      this.error = false
      let data = {
        username: this.$store.state.register__username,
        email: this.$store.state.register__email,
        password: Buffer.from(this.$store.state.register__password)
      }

      var salt = Buffer.from('Sudoku')

      scrypt(data.password, salt, Math.pow(2, 14), 8, 1, 64, (error, progress, key) => {
        if (error) {
          this.error = true
          this.errorMessage = 'Error with password encryption'
        } else if (key) {
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
        }
      })
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
