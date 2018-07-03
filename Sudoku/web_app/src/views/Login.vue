<template lang="pug">
.Login
  ColumnPanel
    Input(placeholder="Username" type="text" id="login__username")
    Input(placeholder="Password" type="password" id="login__password")
    Button(@clicked="login") Login
</template>

<script>
import Input from '@/components/Input.vue'
import ColumnPanel from '@/components/ColumnPanel.vue'
import Button from '@/components/Button.vue'
import axios from 'axios'

export default {
  name: 'Login',
  components: {
    ColumnPanel, Input, Button
  },
  methods: {
    login: function (event) {
      let data = {
        username: this.$store.state.login__username,
        password: this.$store.state.login__password
      }
      
      axios.post('/api/login', data)
      .then(function (response) {
        // Take user to the login page
      })
      .catch(function (error) {
        switch (error.response.status) {
          case 404:
            console.log('Something went horribly wrong')
            break
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
