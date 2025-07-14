<template>
  <div class="settings">
    <div class="page-header">
      <h2>{{ $t('settings.title') }}</h2>
    </div>

    <a-row :gutter="20">
      <a-col :span="12">
        <a-card :title="$t('settings.basic')">
          <a-form :model="systemForm" :label-col="{ span: 8 }" @finish="handleSaveSystemSettings">
            <a-form-item :label="$t('settings.system_name')" name="system_name">
              <a-input v-model:value="systemForm.system_name" />
            </a-form-item>
            <a-form-item :label="$t('settings.system_description')" name="system_description">
              <a-textarea
                v-model:value="systemForm.system_description"
                :rows="4"
              />
            </a-form-item>
            <a-form-item :label="$t('settings.system_version')" name="system_version">
              <a-input v-model:value="systemForm.system_version" />
            </a-form-item>
            <a-form-item>
              <a-button type="primary" html-type="submit" :loading="saving">
                {{ $t('common.save') }}
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>

      <a-col :span="12">
        <a-card :title="$t('settings.ui')">
          <a-form :model="uiForm" :label-col="{ span: 8 }" @finish="handleSaveUISettings">
            <a-form-item :label="$t('settings.theme_color')" name="theme_color">
              <a-select v-model:value="uiForm.theme_color">
                <a-select-option value="#1890ff">蓝色 / Blue</a-select-option>
                <a-select-option value="#52c41a">绿色 / Green</a-select-option>
                <a-select-option value="#722ed1">紫色 / Purple</a-select-option>
                <a-select-option value="#eb2f96">粉色 / Pink</a-select-option>
              </a-select>
            </a-form-item>
            <a-form-item :label="$t('settings.language')" name="language">
              <a-select v-model:value="uiForm.language">
                <a-select-option value="zh-CN">中文</a-select-option>
                <a-select-option value="en-US">English</a-select-option>
              </a-select>
            </a-form-item>
            <a-form-item :label="$t('settings.page_size')" name="page_size">
              <a-select v-model:value="uiForm.page_size">
                <a-select-option :value="10">10条/页 / 10 per page</a-select-option>
                <a-select-option :value="20">20条/页 / 20 per page</a-select-option>
                <a-select-option :value="50">50条/页 / 50 per page</a-select-option>
              </a-select>
            </a-form-item>
            <a-form-item>
              <a-button type="primary" html-type="submit" :loading="saving">
                {{ $t('common.save') }}
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="20" style="margin-top: 20px;">
      <a-col :span="12">
        <a-card :title="t('settings.security')">
          <a-form :model="securityForm" :label-col="{ span: 8 }" @finish="saveSecuritySettings">
            <a-form-item label="允许注册" name="enable_registration">
              <a-switch v-model:checked="securityForm.enable_registration" />
            </a-form-item>
            <a-form-item label="会话超时(秒)" name="session_timeout">
              <a-input-number 
                v-model:value="securityForm.session_timeout" 
                :min="300" 
                :max="86400"
                style="width: 100%;"
              />
            </a-form-item>
            <a-form-item label="最大登录尝试" name="max_login_attempts">
              <a-input-number 
                v-model:value="securityForm.max_login_attempts" 
                :min="3" 
                :max="20"
                style="width: 100%;"
              />
            </a-form-item>
            <a-form-item label="锁定时长(秒)" name="lockout_duration">
              <a-input-number 
                v-model:value="securityForm.lockout_duration" 
                :min="60" 
                :max="3600"
                style="width: 100%;"
              />
            </a-form-item>
            <a-form-item label="重置间隔(秒)" name="reset_attempts_after">
              <a-input-number 
                v-model:value="securityForm.reset_attempts_after" 
                :min="300" 
                :max="86400"
                style="width: 100%;"
              />
            </a-form-item>
            <a-form-item label="维护模式" name="maintenance_mode">
              <a-switch v-model:checked="securityForm.maintenance_mode" />
            </a-form-item>
            <a-form-item>
              <a-button type="primary" html-type="submit" :loading="saving">
                保存设置
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>

      <a-col :span="12">
        <a-card :title="$t('settings.system_info')">
          <a-descriptions bordered>
            <a-descriptions-item :label="$t('settings.system_version')">{{ systemInfo.system_version }}</a-descriptions-item>
            <a-descriptions-item :label="$t('settings.build_time')">2024-01-01</a-descriptions-item>
            <a-descriptions-item :label="$t('settings.uptime')">{{ uptime }}</a-descriptions-item>
            <a-descriptions-item :label="$t('settings.user_count')">{{ userCount }}</a-descriptions-item>
            <a-descriptions-item :label="$t('settings.product_count')">{{ productCount }}</a-descriptions-item>
            <a-descriptions-item :label="$t('settings.order_count')">{{ orderCount }}</a-descriptions-item>
          </a-descriptions>
        </a-card>
      </a-col>
    </a-row>

    <!-- 添加登录日志查看 -->
    <a-row :gutter="20" style="margin-top: 20px;">
      <a-col :span="24">
        <a-card :title="t('settings.login_logs')">
          <a-table
            :columns="logColumns"
            :dataSource="loginLogs"
            :pagination="logPagination"
            :loading="logLoading"
            @change="handleLogTableChange"
            rowKey="id"
          >
            <template #bodyCell="{ column, record }">
              <template v-if="column.key === 'success'">
                <a-tag :color="record.success === 1 ? 'green' : 'red'">
                  {{ record.success === 1 ? t('common.success') : t('common.failed') }}
                </a-tag>
              </template>
              <template v-else-if="column.key === 'created_at'">
                {{ new Date(record.created_at).toLocaleString() }}
              </template>
            </template>
          </a-table>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { message } from 'ant-design-vue'
import { useSettingsStore } from '@/stores/settings'
import { api } from '@/api'

const { t } = useI18n()
const settingsStore = useSettingsStore()
const saving = ref(false)

const systemForm = reactive({
  system_name: 'Rust Admin',
  system_description: '基于 Tauri + Vue 3 的后台管理系统',
  system_version: '1.0.0'
})

const uiForm = reactive({
  theme_color: '#1890ff',
  language: 'zh-CN',
  page_size: 10
})

// 安全设置表单
const securityForm = reactive({
  enable_registration: false,
  session_timeout: 3600,
  max_login_attempts: 5,
  lockout_duration: 300,
  reset_attempts_after: 3600,
  maintenance_mode: false
})

const systemInfo = reactive({
  system_version: '1.0.0'
})

const uptime = ref('0天0小时0分钟')
const userCount = ref(0)
const productCount = ref(0)
const orderCount = ref(0)

// 登录日志
const loginLogs = ref([])
const logLoading = ref(false)
const logPagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0
})

const logColumns = computed(() => [
  {
    title: '用户名',
    dataIndex: 'username',
    key: 'username'
  },
  {
    title: 'IP地址',
    dataIndex: 'ip_address',
    key: 'ip_address'
  },
  {
    title: '状态',
    key: 'success'
  },
  {
    title: '失败原因',
    dataIndex: 'failure_reason',
    key: 'failure_reason'
  },
  {
    title: '时间',
    key: 'created_at'
  }
])

const loadAllSettings = async () => {
  const data = await settingsStore.loadSettings()
  if (data) {
    Object.assign(systemForm, data.system)
    Object.assign(uiForm, data.ui)
    Object.assign(securityForm, data.security)
  }
}

const handleSaveSystemSettings = async () => {
  saving.value = true
  try {
    await settingsStore.saveSystemSettings(systemForm)
  } finally {
    saving.value = false
  }
}

const handleSaveUISettings = async () => {
  saving.value = true
  try {
    const success = await settingsStore.saveUISettings(uiForm)
    if (success) {
      // 立即切换语言
      // locale.value = uiForm.language // This line was removed as per the new_code
    }
  } finally {
    saving.value = false
  }
}

const saveSecuritySettings = async () => {
  saving.value = true
  try {
    await settingsStore.saveSecuritySettings(securityForm)
  } finally {
    saving.value = false
  }
}

const loadSystemInfo = async () => {
  try {
    const response = await api.getSystemInfo()
    if (response.success) {
      const data = response.data
      uptime.value = data.uptime
      userCount.value = data.user_count
      productCount.value = data.product_count
      orderCount.value = data.order_count
    }
  } catch (error) {
    // message.error('加载系统信息失败') // Original code had this line commented out
  }
}

const loadLoginLogs = async () => {
  try {
    const response = await loginLogsApi.getLoginLogs({ page: 1, per_page: 10 })
    if (response.success) {
      // 处理成功的响应
      console.log('Login logs loaded:', response.data)
    } else {
      console.error('Failed to load login logs:', response.message)
    }
  } catch (error) {
    console.error('settings.load_login_logs_failed:', error)
    // 可以显示用户友好的错误消息
  }
}

const handleLogTableChange = (pagination) => {
  logPagination.current = pagination.current
  logPagination.pageSize = pagination.pageSize
  loadLoginLogs()
}

onMounted(() => {
  loadAllSettings()
  loadSystemInfo()
  loadLoginLogs()
})
</script>

<style scoped>
.settings {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}
</style> 