<template>
  <div class="orders">
    <div class="page-header">
      <h2>订单管理</h2>
    </div>

    <a-card>
      <div class="search-form">
        <a-form layout="inline" :model="searchForm">
          <a-form-item>
            <a-input
              v-model:value="searchForm.search"
              placeholder="搜索订单号"
              style="width: 200px;"
            />
          </a-form-item>
          <a-form-item>
            <a-select
              v-model:value="searchForm.status"
              placeholder="订单状态"
              style="width: 120px;"
              allowClear
            >
              <a-select-option value="pending">待付款</a-select-option>
              <a-select-option value="paid">已付款</a-select-option>
              <a-select-option value="shipped">已发货</a-select-option>
              <a-select-option value="delivered">已完成</a-select-option>
              <a-select-option value="cancelled">已取消</a-select-option>
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
        :dataSource="orders"
        :pagination="pagination"
        :loading="loading"
        @change="handleTableChange"
        rowKey="id"
        :expandedRowKeys="expandedRowKeys"
        @expand="handleExpand"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'total_amount'">
            ¥{{ record.total_amount }}
          </template>
          <template v-else-if="column.key === 'status'">
            <a-tag :color="getStatusColor(record.status)">
              {{ getStatusText(record.status) }}
            </a-tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button type="link" @click="viewOrder(record)">查看</a-button>
            <a-select
              v-model:value="record.status"
              @change="updateOrderStatus(record.id, $event)"
              style="width: 100px; margin-left: 8px;"
            >
              <a-select-option value="pending">待付款</a-select-option>
              <a-select-option value="paid">已付款</a-select-option>
              <a-select-option value="shipped">已发货</a-select-option>
              <a-select-option value="delivered">已完成</a-select-option>
              <a-select-option value="cancelled">已取消</a-select-option>
            </a-select>
          </template>
        </template>
        
        <template #expandedRowRender="{ record }">
          <a-table
            :columns="itemColumns"
            :dataSource="record.items"
            :pagination="false"
            size="small"
            rowKey="id"
          >
            <template #bodyCell="{ column, record: item }">
              <template v-if="column.key === 'price'">
                ¥{{ item.price }}
              </template>
              <template v-else-if="column.key === 'total'">
                ¥{{ (item.price * item.quantity).toFixed(2) }}
              </template>
            </template>
          </a-table>
        </template>
      </a-table>
    </a-card>

    <!-- 订单详情弹窗 -->
    <a-modal
      v-model:open="detailModalVisible"
      title="订单详情"
      :footer="null"
      width="800px"
    >
      <div v-if="currentOrder">
        <a-descriptions bordered>
          <a-descriptions-item label="订单号">{{ currentOrder.order_number }}</a-descriptions-item>
          <a-descriptions-item label="用户">{{ currentOrder.user_name }}</a-descriptions-item>
          <a-descriptions-item label="状态">
            <a-tag :color="getStatusColor(currentOrder.status)">
              {{ getStatusText(currentOrder.status) }}
            </a-tag>
          </a-descriptions-item>
          <a-descriptions-item label="总金额">¥{{ currentOrder.total_amount }}</a-descriptions-item>
          <a-descriptions-item label="创建时间">{{ currentOrder.created_at }}</a-descriptions-item>
        </a-descriptions>

        <h3 style="margin: 20px 0;">订单商品</h3>
        <a-table
          :columns="itemColumns"
          :dataSource="currentOrder.items"
          :pagination="false"
          size="small"
          rowKey="id"
        >
          <template #bodyCell="{ column, record: item }">
            <template v-if="column.key === 'price'">
              ¥{{ item.price }}
            </template>
            <template v-else-if="column.key === 'total'">
              ¥{{ (item.price * item.quantity).toFixed(2) }}
            </template>
          </template>
        </a-table>
      </div>
    </a-modal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { api } from '@/api'

const orders = ref([])
const loading = ref(false)
const detailModalVisible = ref(false)
const currentOrder = ref(null)
const expandedRowKeys = ref([])

const searchForm = reactive({
  search: '',
  status: undefined
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
    title: '订单号',
    dataIndex: 'order_number',
    key: 'order_number'
  },
  {
    title: '用户',
    dataIndex: 'user_name',
    key: 'user_name'
  },
  {
    title: '总金额',
    key: 'total_amount',
    width: 100
  },
  {
    title: '状态',
    key: 'status',
    width: 100
  },
  {
    title: '创建时间',
    dataIndex: 'created_at',
    key: 'created_at'
  },
  {
    title: '操作',
    key: 'action',
    width: 200
  }
]

const itemColumns = [
  {
    title: '商品名称',
    dataIndex: 'product_name',
    key: 'product_name'
  },
  {
    title: '单价',
    key: 'price',
    width: 100
  },
  {
    title: '数量',
    dataIndex: 'quantity',
    key: 'quantity',
    width: 80
  },
  {
    title: '小计',
    key: 'total',
    width: 100
  }
]

const loadOrders = async () => {
  loading.value = true
  try {
    const response = await api.getOrders({
      page: pagination.current,
      per_page: pagination.pageSize,
      search: searchForm.search,
      status: searchForm.status
    })
    
    if (response.success) {
      orders.value = response.data.items
      pagination.total = response.data.total
    }
  } catch (error) {
    message.error('加载订单列表失败')
  } finally {
    loading.value = false
  }
}

const getStatusColor = (status) => {
  const colors = {
    pending: 'orange',
    paid: 'blue',
    shipped: 'cyan',
    delivered: 'green',
    cancelled: 'red'
  }
  return colors[status] || 'default'
}

const getStatusText = (status) => {
  const texts = {
    pending: '待付款',
    paid: '已付款',
    shipped: '已发货',
    delivered: '已完成',
    cancelled: '已取消'
  }
  return texts[status] || status
}

const viewOrder = (record) => {
  currentOrder.value = record
  detailModalVisible.value = true
}

const updateOrderStatus = async (orderId, status) => {
  try {
    await api.updateOrderStatus(orderId, status)
    message.success('订单状态更新成功')
    loadOrders()
  } catch (error) {
    message.error('更新失败')
  }
}

const handleExpand = (expanded, record) => {
  if (expanded) {
    expandedRowKeys.value = [record.id]
  } else {
    expandedRowKeys.value = []
  }
}

const handleSearch = () => {
  pagination.current = 1
  loadOrders()
}

const handleReset = () => {
  searchForm.search = ''
  searchForm.status = undefined
  pagination.current = 1
  loadOrders()
}

const handleTableChange = (pag) => {
  pagination.current = pag.current
  pagination.pageSize = pag.pageSize
  loadOrders()
}

onMounted(() => {
  loadOrders()
})
</script>

<style scoped>
.orders {
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