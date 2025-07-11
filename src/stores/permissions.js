import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useAuthStore } from './auth'

export const usePermissionsStore = defineStore('permissions', () => {
  const permissions = ref([])
  const loading = ref(false)

  const authStore = useAuthStore()

  // 检查是否有特定权限
  const hasPermission = computed(() => (permission) => {
    if (!permissions.value.length) {
      // 如果权限还没加载，临时给予基本权限
      return ['dashboard:read'].includes(permission)
    }
    
    // 检查是否有超级权限
    if (permissions.value.includes('*')) return true
    
    // 检查具体权限
    return permissions.value.includes(permission)
  })

  // 检查是否有任一权限
  const hasAnyPermission = computed(() => (permissionList) => {
    return permissionList.some(permission => hasPermission.value(permission))
  })

  // 加载用户权限
  const loadPermissions = async () => {
    if (!authStore.token) {
      permissions.value = []
      return
    }

    loading.value = true
    try {
      const response = await invoke('get_user_permissions', { 
        token: authStore.token 
      })
      
      if (response.success) {
        permissions.value = response.data
      } else {
        // 如果API调用失败，给予基本权限
        permissions.value = ['dashboard:read']
      }
    } catch (error) {
      console.error('Failed to load permissions:', error)
      // 发生错误时，给予基本权限
      permissions.value = ['dashboard:read']
    } finally {
      loading.value = false
    }
  }

  // 清除权限
  const clearPermissions = () => {
    permissions.value = []
  }

  return {
    permissions,
    loading,
    hasPermission,
    hasAnyPermission,
    loadPermissions,
    clearPermissions
  }
}) 