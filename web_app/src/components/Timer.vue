<template lang="pug">
.Timer
  p {{compHours}}h {{compMinutes}}m {{compSeconds}}s {{compMiliseconds}}ms
</template>

<script>
export default {
  name: 'Timer',
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
      })
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
.Timer
  position absolute
  top -70px
  left 0
  width 100%
  height 60px
  font-size 20px
  color #ddd
  text-align center
</style>
