import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'

Vue.config.productionTip = false
Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    input: [],
    userLogged: false,
    selectedSlot: {x: -1, y: -1},
    boardState: Array(9).fill().map(() => Array(9).fill(0))
  },
  mutations: {
    change (input, payload) {
      input[payload.id] = payload.value
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
        store.state.boardState[store.state.selectedSlot.x][store.state.selectedSlot.y] = e.key
      }
      switch (e.key) {
        case 'h':
          store.state.selectedSlot.y--
          break
        case 'j':
          store.state.selectedSlot.x++
          break
        case 'k':
          store.state.selectedSlot.x--
          break
        case 'l':
          store.state.selectedSlot.y++
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
