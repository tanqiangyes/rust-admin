import { createI18n } from 'vue-i18n'

// 直接定义语言包
const zhCN = {
  common: {
    confirm: '确认',
    cancel: '取消',
    save: '保存',
    edit: '编辑',
    delete: '删除',
    add: '新增',
    search: '搜索',
    reset: '重置',
    loading: '加载中...',
    success: '操作成功',
    error: '操作失败'
  },
  menu: {
    dashboard: '仪表盘',
    users: '用户管理',
    products: '商品管理',
    orders: '订单管理',
    categories: '分类管理',
    settings: '系统设置'
  },
  settings: {
    title: '系统设置',
    basic: '基本设置',
    ui: '界面设置',
    security: '安全设置',
    system_info: '系统信息',
    system_name: '系统名称',
    system_description: '系统描述',
    system_version: '系统版本',
    theme_color: '主题色',
    language: '语言',
    page_size: '页面大小',
    enable_registration: '允许注册',
    session_timeout: '会话超时(秒)',
    max_login_attempts: '最大登录尝试',
    maintenance_mode: '维护模式'
  },
  user: {
    username: '用户名',
    email: '邮箱',
    phone: '电话',
    status: '状态',
    created_at: '创建时间'
  }
}

const enUS = {
  common: {
    confirm: 'Confirm',
    cancel: 'Cancel',
    save: 'Save',
    edit: 'Edit',
    delete: 'Delete',
    add: 'Add',
    search: 'Search',
    reset: 'Reset',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error'
  },
  menu: {
    dashboard: 'Dashboard',
    users: 'User Management',
    products: 'Product Management',
    orders: 'Order Management',
    categories: 'Category Management',
    settings: 'System Settings'
  },
  settings: {
    title: 'System Settings',
    basic: 'Basic Settings',
    ui: 'UI Settings',
    security: 'Security Settings',
    system_info: 'System Information',
    system_name: 'System Name',
    system_description: 'System Description',
    system_version: 'System Version',
    theme_color: 'Theme Color',
    language: 'Language',
    page_size: 'Page Size',
    enable_registration: 'Enable Registration',
    session_timeout: 'Session Timeout (seconds)',
    max_login_attempts: 'Max Login Attempts',
    maintenance_mode: 'Maintenance Mode'
  },
  user: {
    username: 'Username',
    email: 'Email',
    phone: 'Phone',
    status: 'Status',
    created_at: 'Created At'
  }
}

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS
}

const i18n = createI18n({
  legacy: false,
  locale: localStorage.getItem('language') || 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages
})

export default i18n