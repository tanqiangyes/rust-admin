<template>
  <div class="orders">
    <div class="page-header">
      <h2>{{ $t('order.title') }}</h2>
    </div>

    <a-card>
      <div class="search-form">
        <a-form layout="inline" :model="searchForm">
          <a-form-item>
            <a-input
              v-model:value="searchForm.search"
              :placeholder="$t('common.search') + $t('order.order_no')"
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
              <a-select-option value="pending">{{ $t('order.pending') }}</a-select-option>
              <a-select-option value="paid">{{ $t('order.paid') }}</a-select-option>
              <a-select-option value="shipped">{{ $t('order.shipped') }}</a-select-option>
              <a-select-option value="delivered">{{ $t('order.delivered') }}</a-select-option>
              <a-select-option value="cancelled">{{ $t('order.cancelled') }}</a-select-option>
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
            <a-button type="link" @click="viewOrder(record)">{{ $t('common.view') }}</a-button>
            <a-select
              v-model:value="record.status"
              @change="updateOrderStatus(record.id, $event)"
              style="width: 100px; margin-left: 8px;"
            >
              <a-select-option value="pending">{{ $t('order.pending') }}</a-select-option>
              <a-select-option value="paid">{{ $t('order.paid') }}</a-select-option>
              <a-select-option value="shipped">{{ $t('order.shipped') }}</a-select-option>
              <a-select-option value="delivered">{{ $t('order.delivered') }}</a-select-option>
              <a-select-option value="cancelled">{{ $t('order.cancelled') }}</a-select-option>
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
      :title="$t('order.order_details')"
      :footer="null"
      width="800px"
    >
      <div v-if="currentOrder">
        <a-descriptions bordered>
          <a-descriptions-item :label="$t('order.order_no')">{{ currentOrder.order_no }}</a-descriptions-item>
          <a-descriptions-item :label="$t('order.user')">{{ currentOrder.username }}</a-descriptions-item>
          <a-descriptions-item :label="$t('common.status')">
            <a-tag :color="getStatusColor(currentOrder.status)">
              {{ getStatusText(currentOrder.status) }}
            </a-tag>
          </a-descriptions-item>
          <a-descriptions-item :label="$t('order.total_amount')">¥{{ currentOrder.total_amount }}</a-descriptions-item>
          <a-descriptions-item :label="$t('common.created_at')">{{ currentOrder.created_at }}</a-descriptions-item>
        </a-descriptions>

        <h3 style="margin: 20px 0;">{{ $t('order.items') }}</h3>
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
import { ref, reactive, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { message } from 'ant-design-vue'
import { api } from '@/api'

const { t } = useI18n()

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

const columns = computed(() => [
  {
    title: t('order.order_no'),
    dataIndex: 'order_no',
    key: 'order_no'
  },
  {
    title: t('order.user'),
    dataIndex: 'username',
    key: 'username'
  },
  {
    title: t('order.total_amount'),
    key: 'total_amount',
    width: 100
  },
  {
    title: t('common.status'),
    key: 'status',
    width: 100
  },
  {
    title: t('common.created_at'),
    dataIndex: 'created_at',
    key: 'created_at'
  },
  {
    title: t('common.action'),
    key: 'action',
    width: 200
  }
])

const itemColumns = computed(() => [
  {
    title: t('product.name'),
    dataIndex: 'product_name',
    key: 'product_name'
  },
  {
    title: t('product.price'),
    key: 'price',
    width: 100
  },
  {
    title: t('order.quantity'),
    dataIndex: 'quantity',
    key: 'quantity',
    width: 80
  },
  {
    title: t('order.subtotal'),
    key: 'total',
    width: 100
  }
])

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
    message.error(t('common.error'))
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
  return t(`order.${status}`)
}

const viewOrder = (record) => {
  currentOrder.value = record
  detailModalVisible.value = true
}

const updateOrderStatus = async (orderId, status) => {
  try {
    await api.updateOrderStatus(orderId, status)
    message.success(t('order.status_updated'))
    loadOrders()
  } catch (error) {
    message.error(t('common.error'))
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