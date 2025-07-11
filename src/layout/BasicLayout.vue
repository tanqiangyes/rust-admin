<template>
  <a-layout style="min-height: 100vh">
    <a-layout-sider v-model:collapsed="collapsed" :trigger="null" collapsible>
      <div class="logo">
        <h3 v-if="!collapsed">{{ $t('app.name') }}</h3>
        <h3 v-else>RA</h3>
      </div>
      <a-menu
        v-model:selectedKeys="selectedKeys"
        v-model:openKeys="openKeys"
        mode="inline"
        theme="dark"
        :items="menuItems"
        @click="handleMenuClick"
      />
    </a-layout-sider>
    
    <a-layout>
      <a-layout-header style="background: #fff; padding: 0; display: flex; justify-content: space-between; align-items: center;">
        <div style="display: flex; align-items: center;">
          <a-button
            type="text"
            :icon="collapsed ? h(MenuUnfoldOutlined) : h(MenuFoldOutlined)"
            @click="() => (collapsed = !collapsed)"
            style="font-size: 16px; width: 64px; height: 64px;"
          />
          <a-breadcrumb>
            <a-breadcrumb-item>{{ currentPageTitle }}</a-breadcrumb-item>
          </a-breadcrumb>
        </div>
        
        <div style="display: flex; align-items: center; margin-right: 20px;">
          <!-- 语言切换 -->
          <a-select
            v-model:value="currentLanguage"
            style="width: 120px; margin-right: 16px;"
            @change="handleLanguageChange"
          >
            <a-select-option value="zh-CN">🇨🇳 中文</a-select-option>
            <a-select-option value="en-US">🇺🇸 English</a-select-option>
          </a-select>
          
          <!-- 用户信息 -->
          <a-dropdown>
            <a-button type="text">
              <a-avatar :src="userInfo.avatar" />
              <span style="margin-left: 8px;">{{ userInfo.username }}</span>
              <down-outlined />
            </a-button>
            <template #overlay>
              <a-menu>
                <a-menu-item key="profile">
                  <user-outlined />
                  {{ $t('user.profile') }}
                </a-menu-item>
                <a-menu-item key="role" disabled>
                  <team-outlined />
                  <a-tag :color="getRoleColor(userInfo.role_name)" size="small">
                    {{ getUserRoleTranslation }}
                  </a-tag>
                </a-menu-item>
                <a-menu-item key="settings" @click="goToSettings">
                  <setting-outlined />
                  {{ $t('menu.settings') }}
                </a-menu-item>
                <a-menu-divider />
                <a-menu-item key="logout" @click="handleLogout">
                  <logout-outlined />
                  {{ $t('auth.logout') }}
                </a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>
        </div>
      </a-layout-header>
      
      <a-layout-content style="margin: 24px 16px; padding: 24px; background: #fff; min-height: 280px;">
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup>
import { ref, reactive, computed, onMounted, watch, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { useSettingsStore } from '@/stores/settings'
import { usePermissionsStore } from '@/stores/permissions'
import { getRoleTranslation, getRoleColor } from '@/utils/roleTranslation'
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  DashboardOutlined,
  UserOutlined,
  ShoppingOutlined,
  ShoppingCartOutlined,
  AppstoreOutlined,
  SettingOutlined,
  DownOutlined,
  LogoutOutlined,
  TeamOutlined
} from '@ant-design/icons-vue'

const router = useRouter()
const route = useRoute()
const { t, locale } = useI18n()
const authStore = useAuthStore()
const settingsStore = useSettingsStore()
const permissionsStore = usePermissionsStore()

const collapsed = ref(false)
const selectedKeys = ref([])
const openKeys = ref([])
const currentLanguage = ref(locale.value)

const userInfo = reactive({
  username: 'admin',
  role_id: 1, // 使用角色ID而不是角色名
  role_name: '超级管理员', // 保留原始名称作为备用
  avatar: 'https://avatars.githubusercontent.com/u/1?v=4'
})

// 计算菜单项（响应语言变化）
const menuItems = computed(() => {
  const items = []
  
  // 仪表盘 - 所有人都能访问
  if (permissionsStore.hasPermission('dashboard:read')) {
    items.push({
      key: '/dashboard',
      icon: h(DashboardOutlined),
      label: t('menu.dashboard'),
      title: t('menu.dashboard')
    })
  }
  
  // 用户管理
  if (permissionsStore.hasPermission('user:read')) {
    items.push({
      key: '/users',
      icon: h(UserOutlined),
      label: t('menu.users'),
      title: t('menu.users')
    })
  }
  
  // 商品管理
  if (permissionsStore.hasPermission('product:read')) {
    items.push({
      key: '/products',
      icon: h(ShoppingOutlined),
      label: t('menu.products'),
      title: t('menu.products')
    })
  }
  
  // 订单管理
  if (permissionsStore.hasPermission('order:read')) {
    items.push({
      key: '/orders',
      icon: h(ShoppingCartOutlined),
      label: t('menu.orders'),
      title: t('menu.orders')
    })
  }
  
  // 分类管理
  if (permissionsStore.hasPermission('category:read')) {
    items.push({
      key: '/categories',
      icon: h(AppstoreOutlined),
      label: t('menu.categories'),
      title: t('menu.categories')
    })
  }
  
  // 系统设置 - 只有超级管理员能访问
  if (permissionsStore.hasPermission('settings:read')) {
    items.push({
      key: '/settings',
      icon: h(SettingOutlined),
      label: t('menu.settings'),
      title: t('menu.settings')
    })
  }
  
  return items
})

// 当前页面标题（响应语言变化）
const currentPageTitle = computed(() => {
  const currentPath = route.path
  const menuItem = menuItems.value.find(item => item.key === currentPath)
  return menuItem ? menuItem.title : t('menu.dashboard')
})

const handleMenuClick = ({ key }) => {
  router.push(key)
}

const handleLanguageChange = async (lang) => {
  locale.value = lang
  currentLanguage.value = lang
  
  // 保存语言设置到后端
  const currentSettings = settingsStore.settings.ui
  await settingsStore.saveUISettings({
    ...currentSettings,
    language: lang
  })
}

const goToSettings = () => {
  router.push('/settings')
}

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}

// 加载当前用户信息
const loadCurrentUser = async () => {
  try {
    // 这里应该从 authStore 或 API 获取当前用户信息
    // 暂时使用硬编码的数据，您可以根据实际情况修改
    if (authStore.user) {
      userInfo.username = authStore.user.username || 'admin'
      userInfo.role_id = authStore.user.role_id || 1 // 假设默认角色ID为1
      userInfo.role_name = authStore.user.role_name || '超级管理员'
      userInfo.avatar = authStore.user.avatar || 'https://avatars.githubusercontent.com/u/1?v=4'
    }
  } catch (error) {
    console.error('Failed to load user info:', error)
  }
}

// 监听路由变化，更新选中的菜单项
watch(
  () => route.path,
  (newPath) => {
    selectedKeys.value = [newPath]
  },
  { immediate: true }
)

// 监听语言变化，更新文档标题
watch(
  () => locale.value,
  () => {
    document.title = settingsStore.settings.system.system_name || t('app.name')
  }
)

// 监听语言变化，重新获取用户信息（如果角色名来自后端且需要翻译）
watch(
  () => locale.value,
  () => {
    // 如果角色名来自后端且是多语言的，这里需要重新获取
    // loadCurrentUser()
  }
)

// 根据角色ID获取翻译
const getUserRoleTranslation = computed(() => {
  // 角色ID到key的映射
  const roleIdMap = {
    1: 'super_admin',
    2: 'admin',
    3: 'user'
  }
  
  const roleKey = roleIdMap[userInfo.role_id] || 'user'
  return t(`role.${roleKey}`)
})

onMounted(async () => {
  // 设置初始选中的菜单项
  selectedKeys.value = [route.path]
  
  // 加载设置
  await settingsStore.loadSettings()
  
  // 设置当前语言
  currentLanguage.value = settingsStore.settings.ui.language
  locale.value = settingsStore.settings.ui.language
  
  // 加载当前用户信息
  await loadCurrentUser()
  
  // 加载权限
  await permissionsStore.loadPermissions()
})
</script>

<style scoped>
.logo {
  height: 32px;
  margin: 16px;
  text-align: center;
  color: white;
}

.logo h3 {
  color: white;
  margin: 0;
  font-size: 18px;
  font-weight: bold;
}

:deep(.ant-layout-sider-children) {
  display: flex;
  flex-direction: column;
}

:deep(.ant-menu) {
  border-right: none;
}

:deep(.ant-layout-header) {
  position: sticky;
  top: 0;
  z-index: 1;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

:deep(.ant-menu-item-disabled) {
  padding-left: 24px !important;
}

:deep(.ant-menu-item-disabled .ant-tag) {
  margin-left: 8px;
}
</style> 