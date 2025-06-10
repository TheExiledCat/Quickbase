import Login from '@/views/Login.vue'
import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/views/Dashboard.vue'
import DashboardEntities from '@/views/Dashboard/DashboardEntities.vue'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: Login,
    },
    {
      path: "/dashboard",
      name: "dashboard",
      redirect: "/dashboard/entities",
      component: Dashboard,
      children: [
        { path: "entities", name: "entities", component: DashboardEntities },
        { path: "metrics", name: "metrics", component: Login },
        { path: "settings", name: "settings", component: Login },
      ]
    }
  ],
})

export default router
