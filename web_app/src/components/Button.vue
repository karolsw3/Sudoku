<template lang="pug">
md-button(@click="$emit('clicked')" :disabled="disabled")
  slot
</template>

<script>
export default {
  name: 'Button',
  props: ['disabled']
}
</script>
