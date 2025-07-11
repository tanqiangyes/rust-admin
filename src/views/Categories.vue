<template>
  <div class="categories">
    <div class="page-header">
      <h2>{{ $t('category.title') }}</h2>
      <a-button type="primary" @click="showCreateModal">
        <template #icon><plus-outlined /></template>
        {{ $t('category.create_category') }}
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
              {{ record.status === 1 ? $t('common.enable') : $t('common.disable') }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="editCategory(record)">{{ $t('common.edit') }}</a-button>
            <a-popconfirm
              :title="$t('category.delete_confirm')"
              @confirm="deleteCategory(record.id)"
            >
              <a-button type="link" danger>{{ $t('common.delete') }}</a-button>
            </a-popconfirm>
          </template>
        </template>
      </a-table>
    </a-card>

    <!-- 新增/编辑分类弹窗 -->
    <a-modal
      v-model:open="modalVisible"
      :title="isEdit ? $t('category.edit_category') : $t('category.create_category')"
      @ok="handleSubmit"
      @cancel="handleCancel"
    >
      <a-form :model="form" :label-col="{ span: 6 }">
        <a-form-item :label="$t('common.name')" name="name">
          <a-input v-model:value="form.name" />
        </a-form-item>
        <a-form-item :label="$t('category.parent_category')" name="parent_id">
          <a-select
            v-model:value="form.parent_id"
            :placeholder="$t('category.parent_category')"
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
        <a-form-item :label="$t('category.sort_order')" name="sort_order">
          <a-input-number
            v-model:value="form.sort_order"
            :min="0"
            style="width: 100%;"
          />
        </a-form-item>
        <a-form-item :label="$t('common.status')" name="status">
          <a-select v-model:value="form.status">
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

const { t } = useI18n()

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

const columns = computed(() => [
  {
    title: 'ID',
    dataIndex: 'id',
    key: 'id',
    width: 80
  },
  {
    title: t('common.name'),
    dataIndex: 'name',
    key: 'name'
  },
  {
    title: t('category.parent_category'),
    dataIndex: 'parent_name',
    key: 'parent_name'
  },
  {
    title: t('category.sort_order'),
    dataIndex: 'sort_order',
    key: 'sort_order',
    width: 80
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

const loadCategories = async () => {
  loading.value = true
  try {
    const response = await api.getCategories()
    if (response.success) {
      categories.value = response.data
    }
  } catch (error) {
    message.error(t('common.error'))
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
      message.success(t('category.category_updated'))
    } else {
      await api.createCategory(form)
      message.success(t('category.category_created'))
    }
    
    modalVisible.value = false
    loadCategories()
  } catch (error) {
    message.error(t('common.error'))
  }
}

const deleteCategory = async (id) => {
  try {
    await api.deleteCategory(id)
    message.success(t('category.category_deleted'))
    loadCategories()
  } catch (error) {
    message.error(t('common.error'))
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