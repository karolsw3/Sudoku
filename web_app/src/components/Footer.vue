<template lang="pug">
.Footer
  span.entry An open-source app by <a href='https://github.com/Galactim' target='_blank'><b>Galactim</b></a>
  span.entry Frontend by <a href='https://github.com/karolsw3' target='_blank'><b>Karol Świerczek</b></a>
  span.entry Backend by <a href='https://nabijaczleweli.xyz' target='_blank'><b>nabijaczleweli</b></a>
  span.entry <a href='https://www.facebook.com/thebestsudoku/' target='_blank'><b>Our Facebook</b></a>
  span.entry
    span.short_span.Footer__led
    span {{activeUsers}} player{{activeUsers === 1 ? '' : 's'}} online
</template>

<script>
import axios from 'axios'

export default {
  name: 'Footer',
  data: function () {
    return {
      activeUsers: 1,
      interval: null
    }
  },
  methods: {
    checkActiveUsers () {
      axios.get('/api/v1/check/active_users')
        .then((response) => {
          this.activeUsers = parseInt(response.data)
        })
        .catch((error) => {
          console.error(error)
        })
    },
    startCheckingActiveUsers () {
      this.interval = setInterval(() => {
        this.checkActiveUsers()
      }, 20000)
    },
    stopCheckingActiveUsers () {
      clearInterval(this.interval)
    },
  },
  created() {
    this.startCheckingActiveUsers()
  }
}
</script>

<style scoped lang="stylus">
.Footer
  width 100%
  display block
  padding 10px 0
  text-align center
  color #999
  bottom 0 !important
  z-index 3
  span
    display inline-block
    margin 0 16px 0 0
  .short_span
    margin 0 4px 0 0
  a
    text-decoration none
  &__led
    width 12px
    height 12px
    background-color #00db83
    border none
    border-radius 25px
    display block
    padding 0px 5px 2px 4px
    position relative
    top 2px
    &:after
      content ''
      width 12px
      height 12px
      position absolute
      top 50%
      left 50%
      transform translateX(-50%) translateY(-50%)
      border 3px solid #00db83
      border-radius 50%
      animation beacon 2s infinite linear
      animation-fill-mode forwards

@keyframes beacon
  0%
  	width 0
    height 0
    opacity 1
  25%
    width 10px
    height 10px
    opacity 0.7
  60%
    width 17px
    height 17px
  100%
    width 20px
    height 20px
    opacity 0
@media screen and (max-width: 700px)
  .entry
    width 100%
  .short_span
    width auto !important
  .Footer
    text-align center

</style>
