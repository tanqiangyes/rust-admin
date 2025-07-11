<template>
  <a-layout class="layout">
    <a-layout-sider
      v-model:collapsed="collapsed"
      collapsible
      width="250"
    >
      <div class="logo">
        <h3>Rust Admin</h3>
      </div>
      <a-menu
        mode="inline"
        :selected-keys="selectedKeys"
        @click="handleMenuClick"
      >
        <a-menu-item key="/dashboard">
          仪表盘
        </a-menu-item>
        <a-menu-item key="/users">
          用户管理
        </a-menu-item>
        <a-menu-item key="/products">
          商品管理
        </a-menu-item>
        <a-menu-item key="/orders">
          订单管理
        </a-menu-item>
        <a-menu-item key="/categories">
          分类管理
        </a-menu-item>
        <a-menu-item key="/settings">
          系统设置
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    
    <a-layout>
      <a-layout-header class="header">
        <div class="header-left">
          <span>后台管理系统</span>
        </div>
        
        <div class="header-right">
          <a-button @click="handleLogout">退出登录</a-button>
        </div>
      </a-layout-header>
      
      <a-layout-content class="content">
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const collapsed = ref(false)
const selectedKeys = computed(() => [route.path])

const handleMenuClick = ({ key }) => {
  router.push(key)
}

const handleLogout = async () => {
  await authStore.logout()
  router.push('/login')
}
</script>

<style scoped>
.layout {
  height: 100vh;
}

.logo {
  height: 64px;
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  background: rgba(255, 255, 255, 0.2);
  margin: 16px;
  border-radius: 6px;
}

.logo h3 {
  margin: 0;
  color: white;
}

.header {
  background: #fff;
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 1px 4px rgba(0, 21, 41, 0.08);
}

.content {
  margin: 24px 16px;
  padding: 24px;
  background: #fff;
  overflow-y: auto;
  border-radius: 6px;
}
</style> 