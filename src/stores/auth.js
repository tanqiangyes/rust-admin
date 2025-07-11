import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '')
  const user = ref(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => !!token.value)

  const login = async (credentials) => {
    loading.value = true
    try {
      if (credentials.username === 'admin' && credentials.password === 'admin123') {
        const mockUser = {
          id: 1,
          username: 'admin',
          email: 'admin@example.com',
          role: 'admin'
        }
        
        token.value = 'mock-token-123'
        user.value = mockUser
        localStorage.setItem('token', token.value)
        localStorage.setItem('user', JSON.stringify(mockUser))
        
        return true
      } else {
        return false
      }
    } catch (error) {
      return false
    } finally {
      loading.value = false
    }
  }

  const logout = async () => {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  const initAuth = () => {
    const savedUser = localStorage.getItem('user')
    const savedToken = localStorage.getItem('token')
    
    if (savedUser && savedToken) {
      user.value = JSON.parse(savedUser)
      token.value = savedToken
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