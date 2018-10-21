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
            :class="[getSelectedClass(i, j, x, y), getLockedClass(i, j, x, y), getHighlightedClass(i, j, x, y), getDimmedClass(i, j, x, y)]"
          )
            p(v-if="slots[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] != 0") {{slots[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)] % 10}}
            .Board__pencilGrid(v-else)
              .slot(v-for='number in 9') {{pencilSlots[getSlotX(i, j, x, y)][getSlotY(i, j, x, y)][number] % 10 ? number:''}}
</template>

<script>

export default {
  name: 'Board',
  data: function () {
    return {
      slots: Array(9).fill().map(() => Array(9).fill(0)),
      pencilSlots: Array(9).fill().map(() => Array(9).fill().map(() => Array(9).fill(false))), // This isn't that hard... trust me!
      filledSlots: 0,
      selectedSlot: {
        x: 0,
        y: 0
      },
      highlightedNumber: 0,
      shiftPressed: false,
      isFilled: false
    }
  },
  props: ['pencilMode'],
  created () {
    window.addEventListener('keydown', this.keydown)
  },
  methods: {
    reset () {
      this.slots = Array(9).fill().map(() => Array(9).fill(0))
      this.pencilSlots = Array(9).fill().map(() => Array(9).fill().map(() => Array(9).fill(false)))
      this.filledSlots = 0
      this.highlightedNumber = 0
      this.shiftPressed = false
      this.isFilled = false
    },
    onSlotClick (x, y) {
      this.selectedSlot.x = x
      this.selectedSlot.y = y
      this.highlightedNumber = this.slots[x][y]
    },
    mutateSelectedSlot (newValue) {
      if (this.slots[this.selectedSlot.x][this.selectedSlot.y] < 10) { // If the value is greater than 9 it means that the slot is locked (see Play.vue)
        if (this.pencilMode) {
          this.pencilSlots[this.selectedSlot.x][this.selectedSlot.y][parseInt(newValue)] = !this.pencilSlots[this.selectedSlot.x][this.selectedSlot.y][parseInt(newValue)]
        } else {
          this.slots[this.selectedSlot.x][this.selectedSlot.y] = parseInt(newValue)
        }
        this.$forceUpdate()
      }
    },
    keydown (e) {
      if (this.$router.currentRoute.name === 'play') {
        if (!isNaN(e.key) && e.key !== ' ') {
          this.checkIfSlotHasBeenFilled(e.key)
          this.mutateSelectedSlot(e.key)
          this.checkIfBoardIsFullyFilled()
        }
        switch (e.key) {
          case 'Backspace':
            this.checkIfSlotHasBeenFilled(0)
            this.mutateSelectedSlot(0)
            break
          case 'h':
          case 'a':
          case 'ArrowLeft':
            this.selectSlotLeft()
            break
          case 'j':
          case 's':
          case 'ArrowDown':
            this.selectSlotDown()
            break
          case 'k':
          case 'w':
          case 'ArrowUp':
            this.selectSlotUp()
            break
          case 'l':
          case 'd':
          case 'ArrowRight':
            this.selectSlotRight()
            break
          case 'H':
          case 'A':
            this.selectSlotLeftWarp()
            break
          case 'J':
          case 'S':
            this.selectSlotDownWarp()
            break
          case 'K':
          case 'W':
            this.selectSlotUpWarp()
            break
          case 'L':
          case 'D':
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
      if (!this.pencilMode) {
        newSlotValue = parseInt(newSlotValue)
        if (newSlotValue > 0 && this.slots[this.selectedSlot.x][this.selectedSlot.y] === 0) {
          this.filledSlots++
        } else if (newSlotValue === 0 && this.slots[this.selectedSlot.x][this.selectedSlot.y] > 0 && this.slots[this.selectedSlot.x][this.selectedSlot.y] < 10) {
          this.filledSlots--
        }
      }
    },
    checkIfBoardIsFullyFilled () {
      if (this.filledSlots === 3 * 3 * 9) {
        // Make an axios request to send the board state
        this.isFilled = true
        if (this.isValid()) {
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
    getHighlightedClass (i, j, x, y) {
      return (this.slots[this.getSlotX(i, j, x, y)][this.getSlotY(i, j, x, y)] % 10) === (this.highlightedNumber % 10) ? 'Board__slot--highlighted' : ''
    },
    getDimmedClass (i, j, x, y) {
      return !((i === (Math.floor(this.selectedSlot.x / 3) + 1) &&
               j === (Math.floor(this.selectedSlot.y / 3) + 1)) ||
               this.getSlotX(i, j, x, y) === this.selectedSlot.x || this.getSlotY(i, j, x, y) === this.selectedSlot.y
             ) ? 'Board__slot--dimmed' : ''
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
        let occurringNumbers = new Set()
        for (let column in this.slots) {
          if (occurringNumbers.has(this.slots[row][column]) || occurringNumbers.has(0)) {
            return false
          }
          occurringNumbers.add(this.slots[row][column])
        }
      }
      return true
    },
    rowsAreValid () {
      // Same story here
      for (let row in this.slots) {
        let occurringNumbers = new Set()
        for (let column in this.slots) {
          if (occurringNumbers.has(this.slots[column][row]) || occurringNumbers.has(0)) {
            return false
          }
          occurringNumbers.add(this.slots[column][row])
        }
      }
      return true
    },
    gridsAreValid () {
      // Avoid triple loops they said... but what about quadruple ones?
      for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
          let occurringNumbers = new Set()
          for (let x = 0; x < 3; x++) {
            for (let y = 0; y < 3; y++) {
              if (occurringNumbers.has(this.slots[i * 3 + x][j * 3 + y]) || occurringNumbers.has(0)) {
                return false
              }
              occurringNumbers.add(this.slots[i * 3 + x][j * 3 + y])
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
  margin 0 auto
  box-sizing border-box
  user-select none
  &__grid
    position relative
    width 100%
    height 100%
    display grid
    box-sizing border-box
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    padding 1px
    border-radius 5px
    background #a3a3a3
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
  &__pencilGrid
    display grid
    position absolute
    top 0
    right 0
    width 100%
    height 100%
    grid-template 1fr 1fr 1fr / 1fr 1fr 1fr
    font-size 9px
    color #aaa
    .slot
      margin 0
      padding 0
      max-height 16px
      max-width 16px
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
    border 1px solid #ccc
    p
      position absolute
      font-family 'Open Sans', sans-serif
      font-weight 700
      top -5px
      left 0
      right 0
      z-index 2
    &:hover
      background #eeeeee
    &--selected
      box-shadow inset 0 0 0 3px
    &--highlighted
      text-shadow 0 0 13px
    &--dimmed
      opacity .4
    &--locked
      background #eee
      color #333
@media screen and (max-width: 560px)
  .Board
    margin-top 60px
    &__slot
      font-size 1rem
  .Board__grid--main
    width 90vw
    height 90vw
    grid-gap 3px
</style>
