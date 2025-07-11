import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { message } from 'ant-design-vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '')
  const user = ref(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  const login = async (credentials) => {
    loading.value = true
    try {
      console.log('Attempting login with:', credentials.username)
      
      const response = await invoke('login', { request: credentials })
      console.log('Login response:', response)
      
      if (response.success) {
        token.value = response.data.token
        user.value = response.data.user
        localStorage.setItem('token', response.data.token)
        localStorage.setItem('user', JSON.stringify(response.data.user))
        console.log('Login successful, user:', response.data.user)
        return true
      } else {
        console.error('Login failed:', response.message)
        message.error(response.message || '登录失败')
        return false
      }
    } catch (error) {
      console.error('Login error:', error)
      message.error('登录失败: ' + (error.message || error))
      return false
    } finally {
      loading.value = false
    }
  }

  const logout = () => {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  const initAuth = () => {
    const savedUser = localStorage.getItem('user')
    const savedToken = localStorage.getItem('token')
    
    if (savedUser && savedToken) {
      try {
        user.value = JSON.parse(savedUser)
        token.value = savedToken
        console.log('Auth initialized with saved user:', user.value)
      } catch (error) {
        console.error('Failed to parse saved user:', error)
        logout()
      }
    }
  }

  return {
    token,
    user,
    loading,
    isAuthenticated,
    login,
    logout,
    initAuth
  }
}) 