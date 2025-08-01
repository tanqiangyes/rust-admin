import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/layout/BasicLayout.vue'),
    meta: { requiresAuth: true },
    redirect: '/dashboard',
    children: [
      {
        path: '/dashboard',
        name: 'Dashboard',
        component: () => import('@/views/Dashboard.vue'),
        meta: { 
          title: '仪表盘',
          permissions: ['dashboard:read']
        }
      },
      {
        path: '/users',
        name: 'Users',
        component: () => import('@/views/Users.vue'),
        meta: { 
          title: '用户管理',
          permissions: ['user:read']
        }
      },
      {
        path: '/products',
        name: 'Products',
        component: () => import('@/views/Products.vue'),
        meta: { 
          title: '商品管理',
          permissions: ['product:read']
        }
      },
      {
        path: '/orders',
        name: 'Orders',
        component: () => import('@/views/Orders.vue'),
        meta: { 
          title: '订单管理',
          permissions: ['order:read']
        }
      },
      {
        path: '/categories',
        name: 'Categories',
        component: () => import('@/views/Categories.vue'),
        meta: { 
          title: '分类管理',
          permissions: ['category:read']
        }
      },
      {
        path: '/settings',
        name: 'Settings',
        component: () => import('@/views/Settings.vue'),
        meta: { 
          title: '系统设置',
          permissions: ['settings:read']
        }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router