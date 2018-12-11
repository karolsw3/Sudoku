<template lang="pug">
md-field(:class="{'md-invalid' : invalid}")
  label {{placeholder}}
  md-input(:type="type" v-model="value" :class="{'md-invalid' : invalid}")
  span.md-error() {{errorMessage}}
</template>

<script>
import validator from 'validator'

export default {
  name: 'FormInput',
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
