<template lang="pug">
.Login
  ColumnPanel
    Input(placeholder="Username" type="text" id="login__username")
    Input(placeholder="Password" type="password" id="login__password")
    ErrorMessageBox(v-if="error") {{errorMessage}}
    Button(@clicked="login") Login
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import ErrorMessageBox from '@/components/ErrorMessageBox.vue'
import Button from '@/components/Button.vue'
import axios from 'axios'

export default {
  name: 'Login',
  components: {
    ColumnPanel, Input, Button, ErrorMessageBox
  },
  data: function () {
    return {
      error: false,
      errorMessage: ''
    }
  },
  methods: {
    login: function (event) {

      let data = {
        username: this.$store.state.login__username,
        password: this.$store.state.login__password
      }

      axios.post('/api/login', data)
        .then((response) => {
          // Take user to the login page
        })
        .catch((error) => {
          switch (error.response.status) {
            case 404:
              this.error = true
              this.errorMessage = "Error 404"
              break
            default:
              this.error = true
              this.errorMessage = "Internal server error"
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
