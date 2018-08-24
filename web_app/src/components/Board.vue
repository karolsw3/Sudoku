<template lang="pug">
.Board
  slot
  .Board__grid.Board__grid--main
    template(v-for="i in 3")
      .Board__grid(v-for="j in 3" :class="getOnValidationClass()")
        template(v-for="x in 3")
          .Board__slot(
            v-for="y in 3"
            @click="onSlotClick(getSlotX(i, j, x, y), getSlotY(i, j, x, y))"
            :class="[getSelectedClass(i, j, x, y), getLockedClass(i, j, x, y)]"
          )
            p(v-if="slots[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] != 0" ) {{slots[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] % 10}}
</template>

<script>

export default {
  name: 'Board',
  data: function () {
    return {
      slots: Array(9).fill().map(() => Array(9).fill(0)),
      filledSlots: 0,
      selectedSlot: {
        x: 0,
        y: 0
      },
      shiftPressed: false,
      isFilled: false
    }
  },
  created () {
    window.addEventListener('keydown', this.keydown)
  },
  methods: {
    onSlotClick (x, y) {
      this.selectedSlot.x = x
      this.selectedSlot.y = y
    },
    mutateSelectedSlot (newValue) {
      if (this.slots[this.selectedSlot.x][this.selectedSlot.y] < 10) { // If the value is greater than 9 it means that the slot is locked (see Play.vue)
        this.slots[this.selectedSlot.x][this.selectedSlot.y] = parseInt(newValue)
      }
    },
    keydown (e) {
      if (this.$router.currentRoute.name === 'play') {
        if (!isNaN(e.key)) {
          this.checkIfSlotHasBeenFilled(e.key)
          this.mutateSelectedSlot(e.key)
        }
        switch (e.key) {
          case 'h':
            this.selectSlotLeft()
            break
          case 'j':
            this.selectSlotDown()
            break
          case 'k':
            this.selectSlotUp()
            break
          case 'l':
            this.selectSlotRight()
            break
          case 'H':
            this.selectSlotLeftWarp()
            break
          case 'J':
            this.selectSlotDownWarp()
            break
          case 'K':
            this.selectSlotUpWarp()
            break
          case 'L':
            this.selectSlotRightWarp()
            break
        }
      }
    },
    selectSlotUp () {
      if (this.selectedSlot.x > 0) {
        this.selectedSlot.x--
      }
    },
    selectSlotDown () {
      if (this.selectedSlot.x < 8) {
        this.selectedSlot.x++
      }
    },
    selectSlotLeft () {
      if (this.selectedSlot.y > 0) {
        this.selectedSlot.y--
      }
    },
    selectSlotRight () {
      if (this.selectedSlot.y < 8) {
        this.selectedSlot.y++
      }
    },
    selectSlotUpWarp () {
      if (this.selectedSlot.x > 0) {
        this.selectedSlot.x--
        if (this.slots[this.selectedSlot.x][this.selectedSlot.y] > 9) {
          this.selectSlotUpWarp()
        }
      } else {
        this.selectSlotDownWarp()
      }
    },
    selectSlotDownWarp () {
      if (this.selectedSlot.x < 8) {
        this.selectedSlot.x++
        if (this.slots[this.selectedSlot.x][this.selectedSlot.y] > 9) {
          this.selectSlotDownWarp()
        }
      } else {
        this.selectSlotUpWarp()
      }
    },
    selectSlotLeftWarp () {
      if (this.selectedSlot.y > 0) {
        this.selectedSlot.y--
        if (this.slots[this.selectedSlot.x][this.selectedSlot.y] > 9) {
          this.selectSlotLeftWarp()
        }
      } else {
        this.selectSlotRightWarp()
      }
    },
    selectSlotRightWarp () {
      if (this.selectedSlot.y < 8) {
        this.selectedSlot.y++
        if (this.slots[this.selectedSlot.x][this.selectedSlot.y] > 9) {
          this.selectSlotRightWarp()
        }
      } else {
        this.selectSlotLeftWarp()
      }
    },
    checkIfSlotHasBeenFilled (newSlotValue) {
      if (newSlotValue > 0 && this.slots[this.selectedSlot.x][this.selectedSlot.y] === 0) {
        this.filledSlots++
        this.checkIfBoardIsFullyFilled()
      }
    },
    checkIfBoardIsFullyFilled () {
      if (this.filledSlots === 3 * 3 * 9) {
        // Make an axios request to send the board state
        this.isFilled = true
        if (this.isValid() || true) {
          // Inform parent that the board is ready to be sent to the server
          this.$emit('board-is-valid', this.slots)
        }
      }
    },
    getSlotX (i, j, x, y) {
      return (i - 1) * 3 + (x - 1)
    },
    getSlotY (i, j, x, y) {
      return (j - 1) * 3 + (y - 1)
    },
    getSelectedClass (i, j, x, y) {
      return this.getSlotX(i, j, x, y) === this.selectedSlot.x && this.getSlotY(i, j, x, y) === this.selectedSlot.y ? 'Board__slot--selected' : ''
    },
    getLockedClass (i, j, x, y) {
      return this.slots[this.getSlotX(i, j, x, y)][this.getSlotY(i, j, x, y)] > 10 ? 'Board__slot--locked' : ''
    },
    getOnValidationClass () {
      if (!this.isFilled) {
        return ''
      }
      if (this.isValid()) {
        return 'Board__grid--valid'
      } else {
        return 'Board__grid--invalid'
      }
    },
    lockSlots () { // Locks all currently filled slots
      for (let row in this.slots) {
        for (let column in this.slots) {
          if (this.slots[row][column] > 0) {
            this.slots[row][column] += 10 // If value is greater than > 10 that means that the slot is locked
          }
        }
      }
    },
    countFilledSlots () {
      for (let row in this.slots) {
        for (let column in this.slots[row]) {
          if (this.slots[row][column] > 0) {
            this.filledSlots++
          }
        }
      }
    },
    isValid () {
      // This is why I love coding..
      return (this.columnsAreValid() && this.rowsAreValid() && this.gridsAreValid())
    },
    columnsAreValid () {
      // Lemme check for any repeating numbers..
      for (let row in this.slots) {
        let occuringNumbers = new Set()
        for (let column in this.slots) {
          if (occuringNumbers.has(this.slots[row][column]) || occuringNumbers.has(0)) {
            return false
          }
          occuringNumbers.add(this.slots[row][column])
        }
      }
      return true
    },
    rowsAreValid () {
      // Same story here
      for (let row in this.slots) {
        let occuringNumbers = new Set()
        for (let column in this.slots) {
          if (occuringNumbers.has(this.slots[column][row]) || occuringNumbers.has(0)) {
            return false
          }
          occuringNumbers.add(this.slots[column][row])
        }
      }
      return true
    },
    gridsAreValid () {
      // Avoid triple loops they said... but what about quadruple ones?
      for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
          let occuringNumbers = new Set()
          for (let x = 0; x < 3; x++) {
            for (let y = 0; y < 3; y++) {
              if (occuringNumbers.has(this.slots[i * 3 + x][j * 3 + y]) || occuringNumbers.has(0)) {
                return false
              }
              occuringNumbers.add(this.slots[i * 3 + x][j * 3 + y])
            }
          }
        }
      }
      return true
    }
  }
}
</script>

<style scoped lang="stylus">
.Board
  display inline-block
  position relative
  margin 110px auto 0 auto
  box-sizing border-box
  &__grid
    position relative
    width 100%
    height 100%
    display grid
    box-sizing border-box
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    padding 2px
    border-radius 5px
    background #e9e9e9
    &--main
      width 504px
      height 504px
      border-radius 6px
      border none
      background #fafafa
      grid-gap 12px
    &--valid
      box-shadow 0 0 3px 1px #44ff75
    &--invalid
      box-shadow 0 0 3px 1px #ff4455
  &__slot
    display block
    position relative
    background white
    box-sizing border-box
    width 100%
    height 100%
    max-height 100%
    max-width 100%
    text-align center
    font-weight 900
    font-size 20px
    color #123ffc
    cursor pointer
    transition-duration .12s
    border 2px solid #e9e9e9
    p
      position absolute
      top -7px
      left 0
      right 0
    &:hover
      background #e6e8eb
    &--selected
      box-shadow inset 0 0 0 3px
    &--locked
      background #f5f5f5 !important
      color #333 !important
</style>
