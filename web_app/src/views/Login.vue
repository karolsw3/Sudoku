<template lang="pug">
.Login
  ColumnPanel
    Input(placeholder="Username" type="text" id="login__username")
    Input(placeholder="Password" type="password" id="login__password")
    ErrorMessageBox(v-if="error") {{errorMessage}}
    Loading(v-if="loading")
    Button(@clicked="login") Login
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import ErrorMessageBox from '@/components/ErrorMessageBox.vue'
import Button from '@/components/Button.vue'
import Loading from '@/components/Loading.vue'
import axios from 'axios'

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
        password: this.$store.state.login__password
      }

      axios.post('/api/login', data)
        .then((response) => {
          this.$store.commit('userLogged', true)
          this.loading = false
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
          this.loading = false
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
