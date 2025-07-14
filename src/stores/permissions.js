import { defineStore } from 'pinia'
import { useAuthStore } from './auth'

export const usePermissionsStore = defineStore('permissions', {
  state: () => ({
    permissions: []
  }),

  getters: {
    hasPermission: (state) => {
      return (permission) => {
        // 检查是否有通配符权限（超级管理员）
        if (state.permissions.includes('*')) {
          return true
        }
        
        // 检查具体权限
        return state.permissions.includes(permission)
      }
    },

    // 添加缺失的 hasAnyPermission 方法
    hasAnyPermission: (state) => {
      return (permissionList) => {
        // 检查是否有通配符权限（超级管理员）
        if (state.permissions.includes('*')) {
          return true
        }
        
        // 检查是否有任一权限
        return permissionList.some(permission => state.permissions.includes(permission))
      }
    }
  },

  actions: {
    async loadPermissions() {
      try {
        const authStore = useAuthStore()
        if (authStore.user && authStore.user.permissions) {
          // 解析权限字符串
          const permissionsStr = authStore.user.permissions
          if (typeof permissionsStr === 'string') {
            this.permissions = JSON.parse(permissionsStr)
          } else {
            this.permissions = permissionsStr
          }
          
          console.log('Loaded permissions:', this.permissions)
        }
      } catch (error) {
        console.error('Failed to load permissions:', error)
        this.permissions = []
      }
    },

    setPermissions(permissions) {
      this.permissions = permissions
    },

    clearPermissions() {
      this.permissions = []
    }
  }
}) 