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
