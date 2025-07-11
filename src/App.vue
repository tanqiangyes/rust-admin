<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from './stores/auth'
import { useSettingsStore } from './stores/settings'
import { usePermissionsStore } from './stores/permissions'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const authStore = useAuthStore()
const settingsStore = useSettingsStore()
const permissionsStore = usePermissionsStore()
const { locale } = useI18n()

onMounted(async () => {
  // 初始化认证
  authStore.initAuth()
  
  // 加载并应用设置
  await settingsStore.loadSettings()
  
  // 应用语言设置
  locale.value = settingsStore.settings.ui.language
  
  // 如果已登录，加载权限
  if (authStore.isAuthenticated) {
    await permissionsStore.loadPermissions()
  }
  
  // 设置路由守卫
  router.beforeEach(async (to, from, next) => {
    // 检查是否需要登录
    if (to.meta.requiresAuth && !authStore.isAuthenticated) {
      next('/login')
      return
    }

    // 如果已登录，跳过登录页面
    if (to.path === '/login' && authStore.isAuthenticated) {
      next('/dashboard')
      return
    }

    // 如果已登录但没有加载权限，先加载权限
    if (authStore.isAuthenticated && !permissionsStore.permissions.length) {
      try {
        await permissionsStore.loadPermissions()
      } catch (error) {
        console.error('Failed to load permissions:', error)
      }
    }

    // 检查页面权限
    if (to.meta.permissions && to.meta.permissions.length > 0) {
      const hasPermission = permissionsStore.hasAnyPermission(to.meta.permissions)
      
      if (!hasPermission) {
        console.warn(`No permission for ${to.path}, redirecting to dashboard`)
        next('/dashboard')
        return
      }
    }

    next()
  })
  
  // 检查当前路径
  if (router.currentRoute.value.path === '/' && !authStore.isAuthenticated) {
    router.push('/login')
  }
})
</script>

<style>
:root {
  --primary-color: #1890ff;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  height: 100vh;
}
</style> 