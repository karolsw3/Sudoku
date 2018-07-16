<template lang="pug">
.InputBox
  input.Input(:placeholder="placeholder" :type="type" v-model="value" :class="{'Input--invalid' : invalid}")
  .InputBox__message(v-if="errorMessage") {{errorMessage}}
</template>

<script>
import validator from 'validator'

export default {
  name: 'Input',
  props: ['placeholder', 'type'],
  data: function () {
    return {
      value: '',
      invalid: false,
      errorMessage: ''
    }
  },
  watch: {
    value: function () {
      switch (this.type) {
        case 'email':
          if (!validator.isEmail(this.value)) {
            this.invalid = true
            this.errorMessage = 'Invalid email'
          } else {
            this.invalid = false
            this.errorMessage = ''
          }
          break
        default:
          if (validator.isEmpty(this.value)) {
            this.invalid = true
            this.errorMessage = 'This input can\'t be empty'
          } else if (!validator.isLength(this.value, {min: 5, max: 40})) {
            this.invalid = true
            this.errorMessage = 'You have to use from 5 to 40 characters'
          } else {
            this.invalid = false
            this.errorMessage = ''
          }
      }
      this.$emit('valueChanged')
    }
  }
}
</script>

<style scoped lang="stylus">
.InputBox
  width 100%
  max-width 440px
  position relative
  text-align left
  margin-top 15px
  &__message
    position absolute
    bottom 3px
    font-size 10px
    color #ff4455
    left 5px
.Input
  width 100%
  box-sizing border-box
  padding 19px 20px
  background #f7f7f7
  color #aaa
  border none
  border-radius 2px
  position relative
  &--invalid
    border 1px solid #ff4455
    color #ff4455
</style>
