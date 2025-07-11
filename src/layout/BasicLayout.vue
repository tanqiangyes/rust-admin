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
                <a-menu-item key="settings">
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
  LogoutOutlined
} from '@ant-design/icons-vue'

const router = useRouter()
const route = useRoute()
const { t, locale } = useI18n()
const authStore = useAuthStore()
const settingsStore = useSettingsStore()

const collapsed = ref(false)
const selectedKeys = ref([])
const openKeys = ref([])
const currentLanguage = ref(locale.value)

const userInfo = reactive({
  username: '管理员',
  avatar: 'https://avatars.githubusercontent.com/u/1?v=4'
})

// 计算菜单项（响应语言变化）
const menuItems = computed(() => [
  {
    key: '/dashboard',
    icon: h(DashboardOutlined),
    label: t('menu.dashboard'),
    title: t('menu.dashboard')
  },
  {
    key: '/users',
    icon: h(UserOutlined),
    label: t('menu.users'),
    title: t('menu.users')
  },
  {
    key: '/products',
    icon: h(ShoppingOutlined),
    label: t('menu.products'),
    title: t('menu.products')
  },
  {
    key: '/orders',
    icon: h(ShoppingCartOutlined),
    label: t('menu.orders'),
    title: t('menu.orders')
  },
  {
    key: '/categories',
    icon: h(AppstoreOutlined),
    label: t('menu.categories'),
    title: t('menu.categories')
  },
  {
    key: '/settings',
    icon: h(SettingOutlined),
    label: t('menu.settings'),
    title: t('menu.settings')
  }
])

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

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
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

onMounted(async () => {
  // 设置初始选中的菜单项
  selectedKeys.value = [route.path]
  
  // 加载设置
  await settingsStore.loadSettings()
  
  // 设置当前语言
  currentLanguage.value = settingsStore.settings.ui.language
  locale.value = settingsStore.settings.ui.language
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
</style> 