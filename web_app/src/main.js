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
    boardState: [[0,0,0,0,0,0,0,0,0], // Well.. that's not the brightest method to
    [0,0,0,0,0,0,0,0,0], // make a 9x9 matrix, but that's the most clear
    [0,0,0,0,0,0,0,0,0], // one.
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0]]
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

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
