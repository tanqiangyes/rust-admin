<template>
  <div class="categories">
    <div class="page-header">
      <h2>分类管理</h2>
      <a-button type="primary" @click="showCreateModal">
        <template #icon><plus-outlined /></template>
        新增分类
      </a-button>
    </div>

    <a-card>
      <a-table
        :columns="columns"
        :dataSource="categories"
        :loading="loading"
        rowKey="id"
        :pagination="false"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'status'">
            <a-tag :color="record.status === 1 ? 'green' : 'red'">
              {{ record.status === 1 ? '启用' : '禁用' }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="editCategory(record)">编辑</a-button>
            <a-popconfirm
              title="确定要删除这个分类吗？"
              @confirm="deleteCategory(record.id)"
            >
              <a-button type="link" danger>删除</a-button>
            </a-popconfirm>
          </template>
        </template>
      </a-table>
    </a-card>

    <!-- 新增/编辑分类弹窗 -->
    <a-modal
      v-model:open="modalVisible"
      :title="isEdit ? '编辑分类' : '新增分类'"
      @ok="handleSubmit"
      @cancel="handleCancel"
    >
      <a-form :model="form" :label-col="{ span: 6 }">
        <a-form-item label="分类名称" name="name">
          <a-input v-model:value="form.name" />
        </a-form-item>
        <a-form-item label="父级分类" name="parent_id">
          <a-select
            v-model:value="form.parent_id"
            placeholder="选择父级分类"
            allowClear
          >
            <a-select-option
              v-for="category in parentCategories"
              :key="category.id"
              :value="category.id"
            >
              {{ category.name }}
            </a-select-option>
          </a-select>
        </a-form-item>
        <a-form-item label="排序" name="sort_order">
          <a-input-number
            v-model:value="form.sort_order"
            :min="0"
            style="width: 100%;"
          />
        </a-form-item>
        <a-form-item label="状态" name="status">
          <a-select v-model:value="form.status">
            <a-select-option :value="1">启用</a-select-option>
            <a-select-option :value="0">禁用</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from 'vue'
import { message } from 'ant-design-vue'
import { PlusOutlined } from '@ant-design/icons-vue'
import { api } from '@/api'

const categories = ref([])
const loading = ref(false)
const modalVisible = ref(false)
const isEdit = ref(false)

const form = reactive({
  id: null,
  name: '',
  parent_id: null,
  sort_order: 0,
  status: 1
})

const parentCategories = computed(() => {
  return categories.value.filter(cat => cat.parent_id === null)
})

const columns = [
  {
    title: 'ID',
    dataIndex: 'id',
    key: 'id',
    width: 80
  },
  {
    title: '分类名称',
    dataIndex: 'name',
    key: 'name'
  },
  {
    title: '父级分类',
    dataIndex: 'parent_name',
    key: 'parent_name'
  },
  {
    title: '排序',
    dataIndex: 'sort_order',
    key: 'sort_order',
    width: 80
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

const loadCategories = async () => {
  loading.value = true
  try {
    const response = await api.getCategories()
    if (response.success) {
      categories.value = response.data
    }
  } catch (error) {
    message.error('加载分类列表失败')
  } finally {
    loading.value = false
  }
}

const showCreateModal = () => {
  isEdit.value = false
  modalVisible.value = true
  resetForm()
}

const editCategory = (record) => {
  isEdit.value = true
  modalVisible.value = true
  Object.assign(form, record)
}

const handleSubmit = async () => {
  try {
    if (isEdit.value) {
      await api.updateCategory(form.id, form)
      message.success('分类更新成功')
    } else {
      await api.createCategory(form)
      message.success('分类创建成功')
    }
    
    modalVisible.value = false
    loadCategories()
  } catch (error) {
    message.error('操作失败')
  }
}

const deleteCategory = async (id) => {
  try {
    await api.deleteCategory(id)
    message.success('分类删除成功')
    loadCategories()
  } catch (error) {
    message.error('删除失败')
  }
}

const handleCancel = () => {
  modalVisible.value = false
  resetForm()
}

const resetForm = () => {
  form.id = null
  form.name = ''
  form.parent_id = null
  form.sort_order = 0
  form.status = 1
}

onMounted(() => {
  loadCategories()
})
</script>

<style scoped>
.categories {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
</style> 