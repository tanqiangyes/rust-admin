import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { message } from 'ant-design-vue'

export const useUsersStore = defineStore('users', () => {
  const users = ref([])
  const total = ref(0)
  const loading = ref(false)

  const fetchUsers = async (params = {}) => {
    loading.value = true
    try {
      const response = await invoke('get_users', params)
      if (response.success) {
        users.value = response.data.items
        total.value = response.data.total
      } else {
        message.error(response.message)
      }
    } catch (error) {
      message.error('获取用户列表失败：' + error)
    } finally {
      loading.value = false
    }
  }

  const createUser = async (userData) => {
    try {
      const response = await invoke('create_user', { request: userData })
      if (response.success) {
        message.success('用户创建成功')
        return true
      } else {
        message.error(response.message)
        return false
      }
    } catch (error) {
      message.error('创建用户失败：' + error)
      return false
    }
  }

  const updateUser = async (id, userData) => {
    try {
      const response = await invoke('update_user', { id, request: userData })
      if (response.success) {
        message.success('用户更新成功')
        return true
      } else {
        message.error(response.message)
        return false
      }
    } catch (error) {
      message.error('更新用户失败：' + error)
      return false
    }
  }

  const deleteUser = async (id) => {
    try {
      const response = await invoke('delete_user', { id })
      if (response.success) {
        message.success('用户删除成功')
        return true
      } else {
        message.error(response.message)
        return false
      }
    } catch (error) {
      message.error('删除用户失败：' + error)
      return false
    }
  }

  return {
    users,
    total,
    loading,
    fetchUsers,
    createUser,
    updateUser,
    deleteUser
  }
}) 