<template lang="pug">
.TheTimer
  p {{compHours}}h {{compMinutes}}m {{compSeconds}}s
</template>

<script>
export default {
  name: 'TheTimer',
  data: function () {
    return {
      miliseconds: 0,
      interval: null
    }
  },
  computed: {
    compMiliseconds () {
      return this.miliseconds % 1000
    },
    compSeconds () {
      return Math.floor(this.miliseconds / 1000) % 60
    },
    compMinutes () {
      return Math.floor(this.miliseconds / 1000 / 60) % 60
    },
    compHours () {
      return Math.floor(this.miliseconds / 1000 / 60 / 60)
    }
  },
  methods: {
    start () {
      let startTime = Date.now()
      this.interval = setInterval(() => {
        this.miliseconds = Date.now() - startTime
      }, 1000)
    },
    stop () {
      clearInterval(this.interval)
    },
    set (miliseconds) {
      this.miliseconds = miliseconds
    }
  }
}
</script>

<style scoped lang="stylus">
.TheTimer
  top -55px
  left 0
  width 100%
  font-size 20px
  color #ddd
  text-align center
  font-family 'Open Sans'
</style>
