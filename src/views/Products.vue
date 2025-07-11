<template>
  <div class="products">
    <div class="page-header">
      <h2>商品管理</h2>
      <a-button type="primary" @click="showCreateModal">
        <template #icon><plus-outlined /></template>
        新增商品
      </a-button>
    </div>

    <a-card>
      <div class="search-form">
        <a-form layout="inline" :model="searchForm">
          <a-form-item>
            <a-input
              v-model:value="searchForm.search"
              placeholder="搜索商品名称"
              style="width: 200px;"
            />
          </a-form-item>
          <a-form-item>
            <a-select
              v-model:value="searchForm.category_id"
              placeholder="分类"
              style="width: 120px;"
              allowClear
            >
              <a-select-option
                v-for="category in categories"
                :key="category.id"
                :value="category.id"
              >
                {{ category.name }}
              </a-select-option>
            </a-select>
          </a-form-item>
          <a-form-item>
            <a-button type="primary" @click="handleSearch">搜索</a-button>
            <a-button @click="handleReset" style="margin-left: 8px;">重置</a-button>
          </a-form-item>
        </a-form>
      </div>

      <a-table
        :columns="columns"
        :dataSource="products"
        :pagination="pagination"
        :loading="loading"
        @change="handleTableChange"
        rowKey="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'image'">
            <a-image 
              v-if="record.images && record.images.length > 0"
              :src="record.images[0]" 
              :width="50" 
              :height="50" 
            />
            <span v-else>无图片</span>
          </template>
          <template v-else-if="column.key === 'price'">
            ¥{{ record.price }}
          </template>
          <template v-else-if="column.key === 'status'">
            <a-tag :color="record.status === 1 ? 'green' : 'red'">
              {{ record.status === 1 ? '上架' : '下架' }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="editProduct(record)">编辑</a-button>
            <a-popconfirm
              title="确定要删除这个商品吗？"
              @confirm="deleteProduct(record.id)"
            >
              <a-button type="link" danger>删除</a-button>
            </a-popconfirm>
          </template>
        </template>
      </a-table>
    </a-card>

    <!-- 新增/编辑商品弹窗 -->
    <a-modal
      v-model:open="modalVisible"
      :title="isEdit ? '编辑商品' : '新增商品'"
      @ok="handleSubmit"
      @cancel="handleCancel"
      width="600px"
    >
      <a-form :model="form" :label-col="{ span: 6 }">
        <a-form-item label="商品名称" name="name">
          <a-input v-model:value="form.name" />
        </a-form-item>
        <a-form-item label="价格" name="price">
          <a-input-number
            v-model:value="form.price"
            :min="0"
            :step="0.01"
            style="width: 100%;"
          />
        </a-form-item>
        <a-form-item label="库存" name="stock">
          <a-input-number
            v-model:value="form.stock"
            :min="0"
            style="width: 100%;"
          />
        </a-form-item>
        <a-form-item label="分类" name="category_id">
          <a-select v-model:value="form.category_id">
            <a-select-option
              v-for="category in categories"
              :key="category.id"
              :value="category.id"
            >
              {{ category.name }}
            </a-select-option>
          </a-select>
        </a-form-item>
        <a-form-item label="描述" name="description">
          <a-textarea
            v-model:value="form.description"
            :rows="4"
          />
        </a-form-item>
        <a-form-item label="状态" name="status">
          <a-select v-model:value="form.status">
            <a-select-option :value="1">上架</a-select-option>
            <a-select-option :value="0">下架</a-select-option>
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

const products = ref([])
const categories = ref([])
const loading = ref(false)
const modalVisible = ref(false)
const isEdit = ref(false)

const searchForm = reactive({
  search: '',
  category_id: undefined
})

const form = reactive({
  id: null,
  name: '',
  price: 0,
  stock: 0,
  category_id: null,
  description: '',
  images: [], // 确保是数组
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
    title: '图片',
    key: 'image',
    width: 80
  },
  {
    title: '商品名称',
    dataIndex: 'name',
    key: 'name'
  },
  {
    title: '价格',
    key: 'price',
    width: 100
  },
  {
    title: '库存',
    dataIndex: 'stock',
    key: 'stock',
    width: 80
  },
  {
    title: '分类',
    dataIndex: 'category_name',
    key: 'category_name'
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

const loadProducts = async () => {
  loading.value = true
  try {
    const response = await api.getProducts({
      page: pagination.current,
      per_page: pagination.pageSize,
      search: searchForm.search,
      category_id: searchForm.category_id
    })
    
    if (response.success) {
      products.value = response.data.items
      pagination.total = response.data.total
    }
  } catch (error) {
    message.error('加载商品列表失败')
  } finally {
    loading.value = false
  }
}

const loadCategories = async () => {
  try {
    const response = await api.getCategories()
    if (response.success) {
      categories.value = response.data
    }
  } catch (error) {
    message.error('加载分类失败')
  }
}

const showCreateModal = () => {
  isEdit.value = false
  modalVisible.value = true
  resetForm()
}

const editProduct = (record) => {
  isEdit.value = true
  modalVisible.value = true
  Object.assign(form, record)
}

const handleSubmit = async () => {
  try {
    if (isEdit.value) {
      await api.updateProduct(form.id, form)
      message.success('商品更新成功')
    } else {
      await api.createProduct(form)
      message.success('商品创建成功')
    }
    
    modalVisible.value = false
    loadProducts()
  } catch (error) {
    message.error('操作失败')
  }
}

const deleteProduct = async (id) => {
  try {
    await api.deleteProduct(id)
    message.success('商品删除成功')
    loadProducts()
  } catch (error) {
    message.error('删除失败')
  }
}

const handleSearch = () => {
  pagination.current = 1
  loadProducts()
}

const handleReset = () => {
  searchForm.search = ''
  searchForm.category_id = undefined
  pagination.current = 1
  loadProducts()
}

const handleTableChange = (pag) => {
  pagination.current = pag.current
  pagination.pageSize = pag.pageSize
  loadProducts()
}

const handleCancel = () => {
  modalVisible.value = false
  resetForm()
}

const resetForm = () => {
  form.id = null
  form.name = ''
  form.price = 0
  form.stock = 0
  form.category_id = null
  form.description = ''
  form.images = [] // 确保是数组
  form.status = 1
}

onMounted(() => {
  loadProducts()
  loadCategories()
})
</script>

<style scoped>
.products {
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