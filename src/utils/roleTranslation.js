import { useI18n } from 'vue-i18n'

// 角色名称映射表（支持多语言输入）
const roleKeyMap = {
  // 中文角色名到 key 的映射
  '超级管理员': 'super_admin',
  '管理员': 'admin',
  '普通用户': 'user',
  '操作员': 'operator',
  '查看者': 'viewer',
  
  // 英文角色名到 key 的映射
  'Super Admin': 'super_admin',
  'Admin': 'admin',
  'User': 'user',
  'Operator': 'operator',
  'Viewer': 'viewer',
  
  // 如果后端存储的是数字ID或者固定key，也可以这样映射
  '1': 'super_admin', // 假设1是超级管理员的ID
  '2': 'admin',
  '3': 'user'
}

// 根据角色名称获取翻译key
export function getRoleKey(roleName) {
  // 如果是数字，转换为字符串
  const key = String(roleName)
  return roleKeyMap[key] || roleKeyMap[roleName] || 'user'
}

// 获取角色的本地化名称
export function getRoleTranslation(roleName, t) {
  const roleKey = getRoleKey(roleName)
  return t(`role.${roleKey}`)
}

// 获取角色颜色
export function getRoleColor(roleName) {
  const roleKey = getRoleKey(roleName)
  const colorMap = {
    'super_admin': 'red',
    'admin': 'orange', 
    'user': 'blue',
    'operator': 'green',
    'viewer': 'gray'
  }
  return colorMap[roleKey] || 'default'
} 