<template>
  <div class="login-container">
    <div class="login-form">
      <div class="login-header">
        <h2>{{ $t('app.name') }}</h2>
        <p>{{ $t('app.description') }}</p>
      </div>
      
      <a-form
        :model="formState"
        @finish="handleLogin"
        :label-col="{ span: 0 }"
        :wrapper-col="{ span: 24 }"
      >
        <a-form-item 
          name="username" 
          :rules="[{ required: true, message: $t('auth.username') + ' ' + $t('common.required') }]"
        >
          <a-input 
            v-model:value="formState.username" 
            :placeholder="$t('auth.username')"
            size="large"
          >
            <template #prefix>
              <user-outlined />
            </template>
          </a-input>
        </a-form-item>
        
        <a-form-item 
          name="password" 
          :rules="[{ required: true, message: $t('auth.password') + ' ' + $t('common.required') }]"
        >
          <a-input-password 
            v-model:value="formState.password" 
            :placeholder="$t('auth.password')"
            size="large"
          >
            <template #prefix>
              <lock-outlined />
            </template>
          </a-input-password>
        </a-form-item>
        
        <a-form-item>
          <a-checkbox v-model:checked="formState.remember">
            {{ $t('auth.remember') }}
          </a-checkbox>
        </a-form-item>
        
        <a-form-item>
          <a-button 
            type="primary" 
            html-type="submit" 
            size="large" 
            :loading="loading"
            style="width: 100%;"
          >
            {{ $t('auth.login') }}
          </a-button>
        </a-form-item>
      </a-form>

      <!-- 语言切换 -->
      <div class="language-switch">
        <a-select
          v-model:value="currentLanguage"
          style="width: 120px;"
          @change="handleLanguageChange"
        >
          <a-select-option value="zh-CN">🇨🇳 中文</a-select-option>
          <a-select-option value="en-US">🇺🇸 English</a-select-option>
        </a-select>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { message } from 'ant-design-vue'
import { UserOutlined, LockOutlined } from '@ant-design/icons-vue'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const { t, locale } = useI18n()
const authStore = useAuthStore()

const loading = ref(false)
const currentLanguage = ref(locale.value)

const formState = reactive({
  username: 'admin',
  password: 'admin123',
  remember: true
})

const handleLogin = async (values) => {
  loading.value = true
  try {
    const success = await authStore.login(values)
    if (success) {
      message.success(t('auth.login_success'))
      router.push('/dashboard')
    } else {
      message.error(t('auth.login_failed'))
    }
  } catch (error) {
    message.error(t('auth.login_failed'))
  } finally {
    loading.value = false
  }
}

const handleLanguageChange = (lang) => {
  locale.value = lang
  currentLanguage.value = lang
  localStorage.setItem('language', lang)
}
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-form {
  background: white;
  padding: 40px;
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  width: 400px;
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-header h2 {
  margin: 0;
  color: #333;
}

.login-header p {
  margin: 8px 0 0 0;
  color: #666;
  font-size: 14px;
}

.language-switch {
  text-align: center;
  margin-top: 16px;
}
</style> 