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

  // 用户管理（需要token）
  async getUsers(params = {}) {
    const token = localStorage.getItem('token')
    return await invoke('get_users', { 
      token,
      ...params 
    })
  },
  
  async createUser(userData) {
    const token = localStorage.getItem('token')
    return await invoke('create_user', { 
      token,
      request: userData 
    })
  },
  
  async updateUser(id, userData) {
    const token = localStorage.getItem('token')
    return await invoke('update_user', { 
      token,
      userId: id, 
      request: userData 
    })
  },
  
  async deleteUser(id) {
    const token = localStorage.getItem('token')
    return await invoke('delete_user', { 
      token,
      userId: id 
    })
  },

  // 商品管理
  async getProducts(params = {}) {
    const token = localStorage.getItem('token')
    return await invoke('get_products', { 
      token,
      ...params 
    })
  },
  
  async createProduct(productData) {
    const token = localStorage.getItem('token')
    return await invoke('create_product', { 
      token,
      request: productData 
    })
  },
  
  async updateProduct(id, productData) {
    const token = localStorage.getItem('token')
    return await invoke('update_product', { 
      token,
      productId: id, 
      request: productData 
    })
  },
  
  async deleteProduct(id) {
    const token = localStorage.getItem('token')
    return await invoke('delete_product', { 
      token,
      productId: id 
    })
  },

  // 订单管理
  async getOrders(params = {}) {
    const token = localStorage.getItem('token')
    return await invoke('get_orders', { 
      token,
      ...params 
    })
  },
  
  async getOrderById(id) {
    const token = localStorage.getItem('token')
    return await invoke('get_order_by_id', { 
      token,
      orderId: id 
    })
  },
  
  async updateOrderStatus(id, status) {
    const token = localStorage.getItem('token')
    return await invoke('update_order_status', { 
      token,
      orderId: id, 
      status 
    })
  },

  // 分类管理
  async getCategories() {
    const token = localStorage.getItem('token')
    return await invoke('get_categories', { token })
  },
  
  async createCategory(categoryData) {
    const token = localStorage.getItem('token')
    return await invoke('create_category', { 
      token,
      request: categoryData 
    })
  },
  
  async updateCategory(id, categoryData) {
    const token = localStorage.getItem('token')
    return await invoke('update_category', { 
      token,
      categoryId: id, 
      request: categoryData 
    })
  },
  
  async deleteCategory(id) {
    const token = localStorage.getItem('token')
    return await invoke('delete_category', { 
      token,
      categoryId: id 
    })
  },

  // 角色管理
  async getRoles() {
    const token = localStorage.getItem('token')
    return await invoke('get_roles', { token })
  },

  // 统计信息
  async getDashboardStats() {
    const token = localStorage.getItem('token')
    return await invoke('get_dashboard_stats', { token })
  },

  async getSystemInfo() {
    const token = localStorage.getItem('token')
    return await invoke('get_system_info', { token })
  },

  // 设置
  async getAllSettings() {
    const token = localStorage.getItem('token')
    return await invoke('get_all_settings', { token })
  },

  async saveSystemSettings(settings) {
    const token = localStorage.getItem('token')
    return await invoke('save_system_settings', { 
      token,
      settings 
    })
  },

  async saveUISettings(settings) {
    const token = localStorage.getItem('token')
    return await invoke('save_ui_settings', { 
      token,
      settings 
    })
  },

  async saveSecuritySettings(settings) {
    return await invoke('save_security_settings', { settings })
  },

  async updateSetting(request) {
    return await invoke('update_setting', { request })
  },

  async getSystemSettings() {
    const token = localStorage.getItem('token')
    return await invoke('get_system_settings', { token })
  }
} 