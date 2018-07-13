import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'

Vue.config.productionTip = false
Vue.use(Vuex)

/*
  Vuex store
*/

const store = new Vuex.Store({
  state: {
    input: [],
    userLogged: false,
    selectedSlot: {x: 0, y: 0},
    boardState: Array(9).fill().map(() => Array(9).fill(0))
  },
  mutations: {
    mutateInput (input, payload) {
      input[payload.id] = payload.value
    },
    mutateBoard (state, payload) {
      state.boardState = payload
    },
    mutateBoardSlot (state, payload) {
      if (state.boardState[payload.x][payload.y] < 10) { // If value is greater than > 10 it means that the slot is locked (see Play.vue)
        state.boardState[payload.x][payload.y] = payload.value
      }
    },
    login (userLogged, boolean) {
      userLogged = boolean
    },
    slotSelected (state, payload) { // When a slot on the board in selected
      state.selectedSlot.x = payload.x
      state.selectedSlot.y = payload.y
    }
  }
})

/*
  Keyboard
*/

document.body.addEventListener('keydown', function (e) {
  switch (router.currentRoute.name) {
    case 'play':
      if (!isNaN(e.key)) {
        store.commit('mutateBoardSlot', {
          x: store.state.selectedSlot.x,
          y: store.state.selectedSlot.y,
          value: e.key
        })
      }
      switch (e.key) {
        case 'h':
          if (store.state.selectedSlot.y > 0) {
            store.state.selectedSlot.y--
          }
          break
        case 'j':
          if (store.state.selectedSlot.x < 8) {
            store.state.selectedSlot.x++
          }
          break
        case 'k':
          if (store.state.selectedSlot.x > 0) {
            store.state.selectedSlot.x--
          }
          break
        case 'l':
          if (store.state.selectedSlot.y < 8) {
            store.state.selectedSlot.y++
          }
          break
      }
      break
  }
}, false)

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
