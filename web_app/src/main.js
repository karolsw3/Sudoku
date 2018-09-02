import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'
import Meta from 'vue-meta'
import { MdButton, MdMenu, MdField, MdList, MdTable, MdContent, MdCard, MdProgress, MdSnackbar, MdRipple } from 'vue-material/dist/components'
import 'vue-material/dist/vue-material.min.css'
import 'vue-material/dist/theme/default.css'

Vue.config.productionTip = false
Vue.use(Vuex)
Vue.use(Meta)

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
  state: {
    user: {
      logged: false,
      login: '',
      email: '',
      pointsTotal: 0,
      gamesTotal: 0,
      isAdmin: false
    }
  },
  mutations: {
    login (state, payload) {
      state.user.logged = true
      state.user.login = payload.login
      state.user.email = payload.email
      state.user.pointsTotal = payload.pointsTotal
      state.user.gamesTotal = payload.gamesTotal
      state.user.isAdmin = payload.isAdmin
    },
    logout (state) {
      state.user.logged = false
      state.user.login = ''
      state.user.email = ''
      state.user.pointsTotal = 0
      state.user.gamesTotal = 0
      state.user.isAdmin = false
    },
    addPoints (state, payload) {
      state.user.pointsTotal += payload
    },
    addGames (state) {
      state.user.gamesTotal++
    }
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')

Vue.use(Meta, {
  keyName: 'metaInfo', // the component option name that vue-meta looks for meta info on.
  attribute: 'data-vue-meta', // the attribute name vue-meta adds to the tags it observes
  ssrAttribute: 'data-vue-meta-server-rendered', // the attribute name that lets vue-meta know that meta info has already been server-rendered
  tagIDKeyName: 'vmid' // the property name that vue-meta uses to determine whether to overwrite or append a tag
})
