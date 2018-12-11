import Vue from 'vue'
import Router from 'vue-router'
import HomePage from './pages/HomePage.vue'
import LoginPage from './pages/LoginPage.vue'
import RegisterPage from './pages/RegisterPage.vue'
import PlayPage from './pages/PlayPage.vue'
import LeaderboardPage from './pages/LeaderboardPage.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomePage
    },
    {
      path: '/login',
      name: 'login',
      component: LoginPage
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterPage
    },
    {
      path: '/play/:difficulty',
      name: 'play',
      component: PlayPage,
      props: true
    },
    {
      path: '/leaderboard/:type',
      name: 'leaderboard',
      component: LeaderboardPage
    },
    { path: '/index.html', redirect: '/' }
  ]
})
