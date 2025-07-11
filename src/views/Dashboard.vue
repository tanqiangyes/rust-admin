<template>
  <div class="dashboard">
    <div class="page-header">
      <h2>{{ $t('dashboard.title') }}</h2>
      <p>{{ $t('dashboard.welcome') }} {{ systemName }}</p>
    </div>

    <!-- 统计卡片 -->
    <a-row :gutter="16" style="margin-bottom: 20px;">
      <a-col :span="6">
        <a-card>
          <a-statistic
            :title="$t('dashboard.total_users')"
            :value="stats.total_users"
            :value-style="{ color: '#3f8600' }"
          >
            <template #prefix>
              <user-outlined />
            </template>
          </a-statistic>
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            :title="$t('dashboard.total_products')"
            :value="stats.total_products"
            :value-style="{ color: '#cf1322' }"
          >
            <template #prefix>
              <shopping-outlined />
            </template>
          </a-statistic>
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            :title="$t('dashboard.total_orders')"
            :value="stats.total_orders"
            :value-style="{ color: '#1890ff' }"
          >
            <template #prefix>
              <shopping-cart-outlined />
            </template>
          </a-statistic>
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            :title="$t('dashboard.total_revenue')"
            :value="stats.total_revenue"
            :precision="2"
            :value-style="{ color: '#722ed1' }"
            prefix="¥"
          />
        </a-card>
      </a-col>
    </a-row>

    <!-- 图表区域 -->
    <a-row :gutter="16">
      <a-col :span="12">
        <a-card :title="$t('dashboard.user_statistics')">
          <p>{{ $t('dashboard.user_statistics') }} Chart Here</p>
        </a-card>
      </a-col>
      <a-col :span="12">
        <a-card :title="$t('dashboard.sales_trend')">
          <p>{{ $t('dashboard.sales_trend') }} Chart Here</p>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '@/stores/settings'
import { api } from '@/api'
import {
  UserOutlined,
  ShoppingOutlined,
  ShoppingCartOutlined
} from '@ant-design/icons-vue'

const { t } = useI18n()
const settingsStore = useSettingsStore()

const systemName = ref('Rust Admin')
const stats = reactive({
  total_users: 0,
  total_products: 0,
  total_orders: 0,
  total_revenue: 0
})

const loadStats = async () => {
  try {
    const response = await api.getDashboardStats()
    if (response.success) {
      Object.assign(stats, response.data)
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

onMounted(async () => {
  // 加载设置
  await settingsStore.loadSettings()
  systemName.value = settingsStore.settings.system.system_name

  // 加载统计数据
  loadStats()
})
</script>

<style scoped>
.dashboard {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.page-header h2 {
  margin: 0;
  margin-bottom: 8px;
}

.page-header p {
  color: #666;
  margin: 0;
}
</style> 