import App from '@/App.vue'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/', component: App },
  { path: '/:pathMatch(.*)*', component: App },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
