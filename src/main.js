import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'
import Antd from 'ant-design-vue'
import App from './App.vue'
import routes from './router'
import i18n from './i18n'

import 'ant-design-vue/dist/reset.css'
import './style.css'

const app = createApp(App)

const pinia = createPinia()
const router = createRouter({
  history: createWebHistory(),
  routes,
})

app.use(pinia)
app.use(router)
app.use(Antd)
app.use(i18n)

app.mount('#app')

// 路由守卫
import { useAuthStore } from './stores/auth'

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  
  if (to.meta.requiresAuth !== false && !authStore.isAuthenticated) {
    next('/login')
  } else if (to.path === '/login' && authStore.isAuthenticated) {
    next('/')
  } else {
    next()
  }
}) 