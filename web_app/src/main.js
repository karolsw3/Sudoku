import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'
import axios from 'axios'
import { MdButton, MdMenu, MdField, MdList, MdTable, MdContent, MdCard, MdProgress, MdSnackbar, MdRipple } from 'vue-material/dist/components'
import 'vue-material/dist/vue-material.min.css'
import 'vue-material/dist/theme/default.css'
import createPersistedState from 'vuex-persistedstate'

Vue.config.productionTip = false
Vue.use(Vuex)

Vue.use(MdButton)
Vue.use(MdMenu)
Vue.use(MdField)
Vue.use(MdList)
Vue.use(MdTable)
Vue.use(MdContent)
Vue.use(MdCard)
Vue.use(MdProgress)
Vue.use(MdSnackbar)
Vue.use(MdRipple)

/*
  Vuex store
*/

const store = new Vuex.Store({
  plugins: [createPersistedState()],
  state: {
    user: {
      logged: false,
      login: '',
      email: '',
      pointsTotal: 0,
      gamesTotal: 0,
      gamesTotalEasy : 0,
      gamesTotalMedium : 0,
      gamesTotalHard : 0,
      isAdmin: false
    },
    nightMode: true
  },
  mutations: {
    login (state, payload) {
      state.user.logged = true
      state.user.login = payload.login
      state.user.email = payload.email
      state.user.pointsTotal = payload.pointsTotal
      state.user.gamesTotal = payload.gamesTotal
      state.user.gamesTotalEasy = payload.gamesTotalEasy
      state.user.gamesTotalMedium = payload.gamesTotalMedium
      state.user.gamesTotalHard = payload.gamesTotalHard
      state.user.isAdmin = payload.isAdmin
    },
    logout (state) {
      state.user.logged = false
      state.user.login = ''
      state.user.email = ''
      state.user.pointsTotal = 0
      state.user.gamesTotal = 0
      state.user.gamesTotalEasy = 0
      state.user.gamesTotalMedium = 0
      state.user.gamesTotalHard = 0
      state.user.isAdmin = false
    },
    addPoints (state, payload) {
      state.user.pointsTotal += payload
    },
    switchNightMode (state) {
      state.nightMode = !state.nightMode
    },
    addGames (state, difficulty) {
      state.user.gamesTotal++
      switch (difficulty) {
        case 1:
          state.user.gamesTotalEasy++
          break
        case 2:
          state.user.gamesTotalMedium++
          break
        case 3:
          state.user.gamesTotalHard++
          break
      }
    }
  }
})

new Vue({
  router,
  store,
  beforeCreate () {
    axios.get('/api/v1/auth/user_data')
      .then((response) => {
        let responseData = response.data
        this.$store.commit('login', {
          login: responseData.username,
          email: responseData.email,
          pointsTotal: responseData.points_total,
          gamesTotal: responseData.games_total,
          gamesTotalEasy: responseData.games_total_easy,
          gamesTotalMedium: responseData.games_total_medium,
          gamesTotalHard: responseData.games_total_hard,
          isAdmin: responseData.is_admin
        })
      })
      this.$forceUpdate()
  },
  render: h => h(App)
}).$mount('#app')
