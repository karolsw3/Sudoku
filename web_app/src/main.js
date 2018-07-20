import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'
import { MdButton, MdMenu, MdField, MdList, MdTable, MdContent, MdCard, MdProgress } from 'vue-material/dist/components'
import 'vue-material/dist/vue-material.min.css'
import 'vue-material/dist/theme/default.css'

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
/*
  Vuex store
*/

const store = new Vuex.Store({
  state: {
    userLogged: false
  },
  mutations: {
    login (userLogged, boolean) {
      userLogged = boolean
    }
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
