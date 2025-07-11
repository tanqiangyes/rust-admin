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
import { useI18n } from 'vue-i18n'

const router = useRouter()
const authStore = useAuthStore()
const settingsStore = useSettingsStore()
const { locale } = useI18n()

onMounted(async () => {
  // 初始化认证
  authStore.initAuth()
  
  // 加载并应用设置
  await settingsStore.loadSettings()
  
  // 应用语言设置
  locale.value = settingsStore.settings.ui.language
  
  // 检查路由
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