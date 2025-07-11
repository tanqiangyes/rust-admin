<template>
  <div class="users">
    <div class="page-header">
      <h2>{{ $t('menu.users') }}</h2>
      <a-button type="primary" @click="showCreateModal">
        <template #icon><plus-outlined /></template>
        {{ $t('common.add') }}用户
      </a-button>
    </div>

    <a-card>
      <div class="search-form">
        <a-form layout="inline" :model="searchForm">
          <a-form-item>
            <a-input
              v-model:value="searchForm.search"
              :placeholder="$t('common.search') + '用户名或邮箱'"
              style="width: 200px;"
            />
          </a-form-item>
          <a-form-item>
            <a-select
              v-model:value="searchForm.status"
              placeholder="状态"
              style="width: 120px;"
              allowClear
            >
              <a-select-option :value="1">启用</a-select-option>
              <a-select-option :value="0">禁用</a-select-option>
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
              {{ record.status === 1 ? '启用' : '禁用' }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'role_name'">
            <a-tag :color="getRoleColor(record.role_name)">
              {{ record.role_name }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="editUser(record)">{{ $t('common.edit') }}</a-button>
            
            <!-- 根据用户是否可删除显示不同的删除按钮 -->
            <a-popconfirm
              v-if="canDelete(record)"
              title="确定要删除这个用户吗？"
              @confirm="deleteUser(record.id)"
            >
              <a-button type="link" danger>{{ $t('common.delete') }}</a-button>
            </a-popconfirm>
            
            <!-- 不可删除的用户显示禁用的删除按钮 -->
            <a-tooltip v-else title="管理员账户不可删除">
              <a-button type="link" danger disabled>{{ $t('common.delete') }}</a-button>
            </a-tooltip>
          </template>
        </template>
      </a-table>
    </a-card>

    <!-- 新增/编辑用户弹窗 -->
    <a-modal
      v-model:open="modalVisible"
      :title="isEdit ? '编辑用户' : '新增用户'"
      @ok="handleSubmit"
      @cancel="handleCancel"
    >
      <a-form :model="form" :label-col="{ span: 6 }">
        <a-form-item label="用户名" name="username">
          <a-input 
            v-model:value="form.username" 
            :disabled="isEdit && isAdminUser(form)"
          />
        </a-form-item>
        <a-form-item label="邮箱" name="email">
          <a-input v-model:value="form.email" />
        </a-form-item>
        <a-form-item label="密码" name="password" v-if="!isEdit">
          <a-input-password v-model:value="form.password" />
        </a-form-item>
        <a-form-item label="电话" name="phone">
          <a-input v-model:value="form.phone" />
        </a-form-item>
        <a-form-item label="地址" name="address">
          <a-input v-model:value="form.address" />
        </a-form-item>
        <a-form-item label="角色" name="role_id">
          <a-select 
            v-model:value="form.role_id"
            :disabled="isEdit && isAdminUser(form)"
          >
            <a-select-option
              v-for="role in roles"
              :key="role.id"
              :value="role.id"
            >
              {{ role.name }}
            </a-select-option>
          </a-select>
        </a-form-item>
        <a-form-item label="状态" name="status">
          <a-select 
            v-model:value="form.status"
            :disabled="isEdit && isAdminUser(form)"
          >
            <a-select-option :value="1">启用</a-select-option>
            <a-select-option :value="0">禁用</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { PlusOutlined } from '@ant-design/icons-vue'
import { api } from '@/api'

const users = ref([])
const roles = ref([])
const loading = ref(false)
const modalVisible = ref(false)
const isEdit = ref(false)

const searchForm = reactive({
  search: '',
  status: undefined
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

const columns = [
  {
    title: '头像',
    key: 'avatar',
    width: 80
  },
  {
    title: '用户名',
    dataIndex: 'username',
    key: 'username'
  },
  {
    title: '邮箱',
    dataIndex: 'email',
    key: 'email'
  },
  {
    title: '电话',
    dataIndex: 'phone',
    key: 'phone'
  },
  {
    title: '角色',
    key: 'role_name',
    width: 100
  },
  {
    title: '状态',
    key: 'status',
    width: 80
  },
  {
    title: '创建时间',
    dataIndex: 'created_at',
    key: 'created_at'
  },
  {
    title: '操作',
    key: 'action',
    width: 150
  }
]

// 检查用户是否可删除
const canDelete = (user) => {
  return user.username !== 'admin' && user.id !== 1 && user.role_id !== 1
}

// 检查是否为管理员用户
const isAdminUser = (user) => {
  return user.username === 'admin' || user.id === 1 || user.role_id === 1
}

// 获取角色颜色
const getRoleColor = (roleName) => {
  const colorMap = {
    '超级管理员': 'red',
    '管理员': 'orange',
    '普通用户': 'blue'
  }
  return colorMap[roleName] || 'default'
}

const loadUsers = async () => {
  loading.value = true
  try {
    const response = await api.getUsers({
      page: pagination.current,
      per_page: pagination.pageSize,
      search: searchForm.search,
      status: searchForm.status
    })
    
    if (response.success) {
      users.value = response.data.items
      pagination.total = response.data.total
    }
  } catch (error) {
    message.error('加载用户列表失败')
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
    message.error('加载角色列表失败')
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
      // 如果是编辑管理员用户，给出提示
      if (isAdminUser(form)) {
        message.warning('管理员账户的敏感信息不可修改')
      }
      
      await api.updateUser(form.id, form)
      message.success('用户更新成功')
    } else {
      await api.createUser(form)
      message.success('用户创建成功')
    }
    
    modalVisible.value = false
    loadUsers()
  } catch (error) {
    message.error('操作失败: ' + (error.message || '未知错误'))
  }
}

const deleteUser = async (id) => {
  try {
    await api.deleteUser(id)
    message.success('用户删除成功')
    loadUsers()
  } catch (error) {
    message.error('删除失败: ' + (error.message || '未知错误'))
  }
}

const handleSearch = () => {
  pagination.current = 1
  loadUsers()
}

const handleReset = () => {
  searchForm.search = ''
  searchForm.status = undefined
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

onMounted(() => {
  loadUsers()
  loadRoles()
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