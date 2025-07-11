# Rust Admin - 基于 Tauri + Vue 3 的桌面后台管理系统

## 📋 项目概述

**该项目完全使用cursor创建（除了本句话），其余均为ai创作生成。**

Rust Admin 是一个现代化的桌面端后台管理系统，采用 Tauri + Vue 3 技术栈开发。它结合了 Rust 的高性能后端和 Vue 3 的现代化前端，提供了完整的用户管理、商品管理、订单管理等企业级功能。

## ✨ 特性

### 🔐 认证与权限
- 用户登录/登出
- 基于角色的权限控制
- 会话管理
- 登录尝试限制

### 👥 用户管理
- 用户增删改查
- 用户状态管理
- 角色分配
- 分页搜索

### 📦 商品管理
- 商品信息管理
- 分类管理
- 库存管理
- 图片上传
- 商品状态控制

### 📋 订单管理
- 订单列表查看
- 订单状态更新
- 订单详情展示
- 订单搜索筛选

### ⚙️ 系统设置
- 系统基本信息配置
- 界面主题切换（蓝色/绿色/紫色/粉色）
- 多语言支持（中文/英文）
- 分页大小设置
- 安全策略配置

### 📊 数据统计
- 实时数据统计
- 图表展示
- 系统信息监控

## 🛠️ 技术栈

### 后端 (Rust)
- **Tauri** - 桌面应用框架
- **SQLx** - 数据库操作
- **SQLite** - 嵌入式数据库
- **Serde** - 序列化/反序列化
- **BCrypt** - 密码加密
- **Tokio** - 异步运行时

### 前端 (Vue 3)
- **Vue 3** - 渐进式框架
- **Vue Router** - 路由管理
- **Pinia** - 状态管理
- **Ant Design Vue** - UI 组件库
- **Vue I18n** - 国际化
- **Vite** - 构建工具

## 📁 项目结构

```
rust-admin/
├── src/                          # Vue 前端源码
│   ├── api/                      # API 接口
│   ├── components/               # 通用组件
│   ├── i18n/                     # 国际化配置
│   │   └── locales/              # 语言包
│   ├── layout/                   # 布局组件
│   ├── router/                   # 路由配置
│   ├── stores/                   # Pinia 状态管理
│   ├── views/                    # 页面组件
│   │   ├── Dashboard.vue         # 仪表盘
│   │   ├── Users.vue             # 用户管理
│   │   ├── Products.vue          # 商品管理
│   │   ├── Orders.vue            # 订单管理
│   │   ├── Categories.vue        # 分类管理
│   │   ├── Settings.vue          # 系统设置
│   │   └── Login.vue             # 登录页面
│   ├── App.vue                   # 根组件
│   └── main.js                   # 入口文件
├── src-tauri/                    # Tauri 后端源码
│   ├── src/
│   │   ├── api/                  # API 处理器
│   │   │   ├── auth.rs           # 认证相关
│   │   │   ├── users.rs          # 用户管理
│   │   │   ├── products.rs       # 商品管理
│   │   │   ├── orders.rs         # 订单管理
│   │   │   ├── categories.rs     # 分类管理
│   │   │   ├── settings.rs       # 系统设置
│   │   │   └── stats.rs          # 数据统计
│   │   ├── database/             # 数据库相关
│   │   │   ├── mod.rs            # 数据库连接
│   │   │   └── migrations.rs     # 数据库迁移
│   │   ├── models/               # 数据模型
│   │   └── main.rs               # 主程序入口
│   ├── Cargo.toml                # Rust 依赖配置
│   └── tauri.conf.json           # Tauri 配置
├── data/                         # 数据库文件目录
├── package.json                  # Node.js 依赖配置
├── vite.config.js                # Vite 配置
└── README.md                     # 项目说明
```

README.md 文件已经创建完成！这个文档详细介绍了整个项目的：

- 项目概述和特性
- 技术栈和架构
- 项目结构
- 安装和运行指南
- 功能说明
- 开发指南
- 贡献方式

您可以根据实际需要调整其中的内容，比如：
- 更新 GitHub 仓库地址
- 添加实际的项目截图
- 修改联系方式
- 补充具体的许可证信息

## 🚀 快速开始

### 环境要求

- **Node.js** >= 16.0.0
- **Rust** >= 1.70.0
- **系统要求**: Windows 10+, macOS 10.15+, 或 Linux

### 安装依赖

```bash
# 克隆项目
git clone https://github.com/your-username/rust-admin.git
cd rust-admin

# 安装前端依赖
npm install

# 安装 Tauri CLI (如果还没安装)
npm install -g @tauri-apps/cli
```

### 开发模式运行

```bash
# 启动开发服务器
npm run tauri dev
```

### 生产构建

```bash
# 构建应用
npm run tauri build
```

## 📝 配置说明

### 数据库配置

项目使用 SQLite 数据库，数据文件存储在 `data/rust-admin.db`。首次运行时会自动创建数据库表并插入初始数据。

### 默认账号

- **用户名**: admin
- **密码**: admin123
- **权限**: 超级管理员

### 环境变量

可以通过环境变量配置一些参数：

```bash
# 数据库路径
DATABASE_URL=sqlite:./data/rust-admin.db

# 服务端口 (开发模式)
VITE_DEV_SERVER_PORT=1420
```

## 🎯 主要功能

### 1. 用户管理
- ✅ 用户列表展示（分页、搜索、筛选）
- ✅ 新增/编辑/删除用户
- ✅ 用户状态管理（启用/禁用）
- ✅ 角色分配

### 2. 商品管理
- ✅ 商品列表展示（分页、搜索、筛选）
- ✅ 新增/编辑/删除商品
- ✅ 商品分类管理
- ✅ 库存管理
- ✅ 商品状态控制（上架/下架）

### 3. 订单管理
- ✅ 订单列表展示
- ✅ 订单状态更新（待付款/已付款/已发货/已完成/已取消）
- ✅ 订单详情查看
- ✅ 订单搜索筛选

### 4. 分类管理
- ✅ 分类列表展示
- ✅ 新增/编辑/删除分类
- ✅ 层级分类支持
- ✅ 分类排序

### 5. 系统设置
- ✅ 系统基本信息设置
- ✅ 主题色切换（实时生效）
- ✅ 多语言切换（中文/英文）
- ✅ 页面大小设置
- ✅ 安全策略配置

## 🎨 界面预览

### 主要页面

- **登录页面**: 简洁的登录界面，支持用户名/密码登录
- **仪表盘**: 数据统计概览，图表展示
- **用户管理**: 完整的用户 CRUD 操作
- **商品管理**: 商品信息管理，支持图片展示
- **订单管理**: 订单状态跟踪，详情查看
- **系统设置**: 个性化配置，实时生效

### 主题支持

支持 4 种主题色：
- 🔵 蓝色主题 (`#1890ff`)
- 🟢 绿色主题 (`#52c41a`)
- 🟣 紫色主题 (`#722ed1`)
- 🩷 粉色主题 (`#eb2f96`)

## 🌐 国际化

支持多语言：
- 🇨🇳 简体中文 (zh-CN)
- 🇺🇸 英文 (en-US)

语言切换实时生效，包括所有界面文本和表单标签。

## 📊 数据库设计

### 主要数据表

- **users** - 用户表
- **roles** - 角色表
- **products** - 商品表
- **categories** - 分类表
- **orders** - 订单表
- **order_items** - 订单商品表
- **system_settings** - 系统设置表
- **logs** - 操作日志表

## �� 开发指南

### 添加新功能

1. **后端 API**:
   ```rust
   // src-tauri/src/api/new_module.rs
   #[tauri::command]
   pub async fn new_function(
       state: State<'_, AppState>,
   ) -> Result<ApiResponse<DataType>, String> {
       // 实现逻辑
   }
   ```

2. **前端页面**:
   ```vue
   <!-- src/views/NewPage.vue -->
   <template>
     <!-- 页面结构 -->
   </template>
   
   <script setup>
   import { api } from '@/api'
   // 页面逻辑
   </script>
   ```

3. **注册路由**:
   ```js
   // src/router/index.js
   {
     path: '/new-page',
     name: 'NewPage',
     component: () => import('@/views/NewPage.vue')
   }
   ```

### 代码规范

- **Rust**: 遵循 Rust 官方代码规范
- **Vue**: 使用 Composition API
- **TypeScript**: 类型安全
- **CSS**: 使用 Scoped CSS

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 优秀的桌面应用框架
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Ant Design Vue](https://antdv.com/) - 企业级 UI 设计语言
- [SQLx](https://github.com/launchbadge/sqlx) - Rust 异步 SQL 工具包

## 📞 联系方式

- 项目地址: [https://github.com/tanqiangyes/rust-admin](https://github.com/tanqiangyes/rust-admin)
- 问题反馈: [Issues](https://github.com/tanqiangyes/rust-admin/issues)

---

⭐ 如果这个项目对您有帮助，请给它一个 Star！