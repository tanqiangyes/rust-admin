import { invoke } from '@tauri-apps/api/tauri'

export const api = {
  // 认证相关
  async login(credentials) {
    return await invoke('login', { request: credentials })
  },
  
  async logout() {
    return await invoke('logout')
  },
  
  async getCurrentUser(userId) {
    return await invoke('get_current_user', { userId })
  },

  // 用户管理
  async getUsers(params = {}) {
    return await invoke('get_users', params)
  },
  
  async createUser(userData) {
    return await invoke('create_user', { request: userData })
  },
  
  async updateUser(id, userData) {
    return await invoke('update_user', { userId: id, request: userData })
  },
  
  async deleteUser(id) {
    return await invoke('delete_user', { userId: id })
  },

  // 商品管理
  async getProducts(params = {}) {
    return await invoke('get_products', params)
  },
  
  async createProduct(productData) {
    return await invoke('create_product', { request: productData })
  },
  
  async updateProduct(id, productData) {
    return await invoke('update_product', { productId: id, request: productData })
  },
  
  async deleteProduct(id) {
    return await invoke('delete_product', { productId: id })
  },

  // 订单管理
  async getOrders(params = {}) {
    return await invoke('get_orders', params)
  },
  
  async getOrderById(id) {
    return await invoke('get_order_by_id', { orderId: id })
  },
  
  async updateOrderStatus(id, status) {
    return await invoke('update_order_status', { orderId: id, status })
  },

  // 分类管理
  async getCategories() {
    return await invoke('get_categories')
  },
  
  async createCategory(categoryData) {
    return await invoke('create_category', { request: categoryData })
  },
  
  async updateCategory(id, categoryData) {
    return await invoke('update_category', { categoryId: id, request: categoryData })
  },
  
  async deleteCategory(id) {
    return await invoke('delete_category', { categoryId: id })
  },

  // 角色管理
  async getRoles() {
    return await invoke('get_roles')
  },

  // 统计信息
  async getDashboardStats() {
    return await invoke('get_dashboard_stats')
  },

  async getSystemInfo() {
    return await invoke('get_system_info')
  },

  // 设置
  async getAllSettings() {
    return await invoke('get_all_settings')
  },

  async saveSystemSettings(settings) {
    return await invoke('save_system_settings', { settings })
  },

  async saveUISettings(settings) {
    return await invoke('save_ui_settings', { settings })
  },

  async saveSecuritySettings(settings) {
    return await invoke('save_security_settings', { settings })
  },

  async updateSetting(request) {
    return await invoke('update_setting', { request })
  },

  async getSystemSettings() {
    return await invoke('get_system_settings')
  }
} 