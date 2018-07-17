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
    userLogged: false,
    boardState: Array(9).fill().map(() => Array(9).fill(0)),
    filledSlots: 0
  },
  mutations: {
    mutateBoard (state, payload) {
      state.boardState = payload
    },
    mutateBoardSlot (state, payload) {
      if (state.boardState[payload.x][payload.y] < 10) { // If the value is greater than 9 it means that the slot is locked (see Play.vue)
        if (payload.value > 0 && state.boardState[payload.x][payload.y] === 0) {
          state.filledSlots++

          // When the board is fully filled
          if (state.filledSlots === 3 * 3 * 9) {
            // Make an axios request to send the baord state
          }
        }
        state.boardState[payload.x][payload.y] = payload.value
      }
    },
    login (userLogged, boolean) {
      userLogged = boolean
    },
    incrementFilledSlotsCounter (filledSlots) {
      filledSlots++
    }
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
