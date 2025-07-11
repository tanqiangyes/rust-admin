<template>
  <div class="users">
    <div class="page-header">
      <h2>{{ $t('user.title') }}</h2>
      <a-button type="primary" @click="showCreateModal">
        <template #icon><plus-outlined /></template>
        {{ $t('user.create_user') }}
      </a-button>
    </div>

    <a-card>
      <div class="search-form">
        <a-form layout="inline" :model="searchForm">
          <a-form-item>
            <a-input
              v-model:value="searchForm.search"
              :placeholder="$t('common.search') + $t('user.username') + '/' + $t('user.email')"
              style="width: 200px;"
            />
          </a-form-item>
          <a-form-item>
            <a-select
              v-model:value="searchForm.status"
              :placeholder="$t('common.status')"
              style="width: 120px;"
              allowClear
            >
              <a-select-option :value="1">{{ $t('common.enable') }}</a-select-option>
              <a-select-option :value="0">{{ $t('common.disable') }}</a-select-option>
            </a-select>
          </a-form-item>
          <a-form-item>
            <a-select
              v-model:value="searchForm.role_id"
              :placeholder="$t('user.role')"
              style="width: 140px;"
              allowClear
            >
              <a-select-option
                v-for="role in roles"
                :key="role.id"
                :value="role.id"
              >
                {{ getRoleTranslation(role.name, $t) }}
              </a-select-option>
            </a-select>
          </a-form-item>
          <a-form-item>
            <a-button type="primary" @click="handleSearch">{{ $t('common.search') }}</a-button>
            <a-button @click="handleReset" style="margin-left: 8px;">{{ $t('common.reset') }}</a-button>
          </a-form-item>
        </a-form>
      </div>

      <a-table
        :columns="columns"
        :dataSource="users"
        :pagination="pagination"
        :loading="loading"
        @change="handleTableChange"
        rowKey="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'avatar'">
            <a-avatar :src="record.avatar" />
          </template>
          <template v-else-if="column.key === 'status'">
            <a-tag :color="record.status === 1 ? 'green' : 'red'">
              {{ record.status === 1 ? $t('common.enable') : $t('common.disable') }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'role_name'">
            <a-tag :color="getRoleColor(record.role_name)">
              {{ getRoleTranslation(record.role_name, $t) }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="editUser(record)">{{ $t('common.edit') }}</a-button>
            
            <a-popconfirm
              v-if="canDelete(record)"
              :title="$t('user.delete_confirm')"
              @confirm="deleteUser(record.id)"
            >
              <a-button type="link" danger>{{ $t('common.delete') }}</a-button>
            </a-popconfirm>
            
            <a-tooltip v-else :title="$t('user.cannot_delete_admin')">
              <a-button type="link" danger disabled>{{ $t('common.delete') }}</a-button>
            </a-tooltip>
          </template>
        </template>
      </a-table>
    </a-card>

    <!-- 新增/编辑用户弹窗 -->
    <a-modal
      v-model:open="modalVisible"
      :title="isEdit ? $t('user.edit_user') : $t('user.create_user')"
      @ok="handleSubmit"
      @cancel="handleCancel"
    >
      <a-form :model="form" :label-col="{ span: 6 }">
        <a-form-item :label="$t('user.username')" name="username">
          <a-input 
            v-model:value="form.username" 
            :disabled="isEdit && isAdminUser(form)"
          />
        </a-form-item>
        <a-form-item :label="$t('user.email')" name="email">
          <a-input v-model:value="form.email" />
        </a-form-item>
        <a-form-item :label="$t('auth.password')" name="password" v-if="!isEdit">
          <a-input-password v-model:value="form.password" />
        </a-form-item>
        <a-form-item :label="$t('user.phone')" name="phone">
          <a-input v-model:value="form.phone" />
        </a-form-item>
        <a-form-item :label="$t('user.address')" name="address">
          <a-input v-model:value="form.address" />
        </a-form-item>
        <a-form-item :label="$t('user.role')" name="role_id">
          <a-select 
            v-model:value="form.role_id"
            :disabled="isEdit && isAdminUser(form)"
          >
            <a-select-option
              v-for="role in roles"
              :key="role.id"
              :value="role.id"
            >
              {{ getRoleTranslation(role.name, $t) }}
            </a-select-option>
          </a-select>
        </a-form-item>
        <a-form-item :label="$t('common.status')" name="status">
          <a-select 
            v-model:value="form.status"
            :disabled="isEdit && isAdminUser(form)"
          >
            <a-select-option :value="1">{{ $t('common.enable') }}</a-select-option>
            <a-select-option :value="0">{{ $t('common.disable') }}</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { message } from 'ant-design-vue'
import { PlusOutlined } from '@ant-design/icons-vue'
import { api } from '@/api'
import { getRoleTranslation, getRoleColor } from '@/utils/roleTranslation'
import { usePermissionsStore } from '@/stores/permissions'

const { t } = useI18n()
const permissionsStore = usePermissionsStore()

const users = ref([])
const roles = ref([])
const loading = ref(false)
const modalVisible = ref(false)
const isEdit = ref(false)

const searchForm = reactive({
  search: '',
  status: undefined,
  role_id: undefined
})

const form = reactive({
  id: null,
  username: '',
  email: '',
  password: '',
  phone: '',
  address: '',
  role_id: null,
  status: 1
})

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
  showSizeChanger: true,
  showQuickJumper: true
})

const columns = computed(() => [
  {
    title: t('user.avatar'),
    key: 'avatar',
    width: 80
  },
  {
    title: t('user.username'),
    dataIndex: 'username',
    key: 'username'
  },
  {
    title: t('user.email'),
    dataIndex: 'email',
    key: 'email'
  },
  {
    title: t('user.phone'),
    dataIndex: 'phone',
    key: 'phone'
  },
  {
    title: t('user.role'),
    key: 'role_name',
    width: 100
  },
  {
    title: t('common.status'),
    key: 'status',
    width: 80
  },
  {
    title: t('common.created_at'),
    dataIndex: 'created_at',
    key: 'created_at'
  },
  {
    title: t('common.action'),
    key: 'action',
    width: 150
  }
])

// 检查用户是否可删除
const canDelete = (user) => {
  return user.username !== 'admin' && user.id !== 1 && user.role_id !== 1
}

// 检查是否为管理员用户
const isAdminUser = (user) => {
  return user.username === 'admin' || user.id === 1 || user.role_id === 1
}

const loadUsers = async () => {
  loading.value = true
  try {
    console.log('Loading users...')
    const response = await api.getUsers({
      page: pagination.current,
      per_page: pagination.pageSize,
      search: searchForm.search,
      status: searchForm.status
    })
    
    console.log('Users API response:', response)
    
    if (response.success) {
      users.value = response.data.items
      pagination.total = response.data.total
    } else {
      message.error(response.message || t('common.error'))
    }
  } catch (error) {
    console.error('Error loading users:', error)
    message.error(t('common.error') + ': ' + (error.message || error))
  } finally {
    loading.value = false
  }
}

const loadRoles = async () => {
  try {
    const response = await api.getRoles()
    if (response.success) {
      roles.value = response.data
    }
  } catch (error) {
    message.error(t('common.error'))
  }
}

const showCreateModal = () => {
  isEdit.value = false
  modalVisible.value = true
  resetForm()
}

const editUser = (record) => {
  isEdit.value = true
  modalVisible.value = true
  Object.assign(form, record)
}

const handleSubmit = async () => {
  try {
    if (isEdit.value) {
      if (isAdminUser(form)) {
        message.warning(t('user.admin_fields_readonly'))
      }
      
      await api.updateUser(form.id, form)
      message.success(t('user.user_updated'))
    } else {
      await api.createUser(form)
      message.success(t('user.user_created'))
    }
    
    modalVisible.value = false
    loadUsers()
  } catch (error) {
    message.error(t('common.error') + ': ' + (error.message || t('common.error')))
  }
}

const deleteUser = async (id) => {
  try {
    await api.deleteUser(id)
    message.success(t('user.user_deleted'))
    loadUsers()
  } catch (error) {
    message.error(t('common.error') + ': ' + (error.message || t('common.error')))
  }
}

const handleSearch = () => {
  pagination.current = 1
  loadUsers()
}

const handleReset = () => {
  searchForm.search = ''
  searchForm.status = undefined
  searchForm.role_id = undefined
  pagination.current = 1
  loadUsers()
}

const handleTableChange = (pag) => {
  pagination.current = pag.current
  pagination.pageSize = pag.pageSize
  loadUsers()
}

const handleCancel = () => {
  modalVisible.value = false
  resetForm()
}

const resetForm = () => {
  form.id = null
  form.username = ''
  form.email = ''
  form.password = ''
  form.phone = ''
  form.address = ''
  form.role_id = null
  form.status = 1
}

// 添加调试信息
onMounted(async () => {
  console.log('Current permissions:', permissionsStore.permissions)
  console.log('Has user:read permission:', permissionsStore.hasPermission('user:read'))
  
  await loadUsers()
  await loadRoles()
})
</script>

<style scoped>
.users {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.search-form {
  margin-bottom: 16px;
}
</style> 