import Login from '@/views/Login.vue'
import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/views/Dashboard.vue'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: Login,
    },
    {
      path:"/dashboard",
      name:"dashboard",
      component: Dashboard
    }
  ],
})

export default router
