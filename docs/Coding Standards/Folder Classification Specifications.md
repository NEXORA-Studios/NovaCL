## **文件夹分类规范**

(root)/
├── src/                    # 💻 前端 Vue 应用源代码
│   ├── api/                # 与 Rust 后端交互的封装
│   ├── components/         # 通用 UI 组件
│   ├── modules/            # 前端功能模块
│   ├── pages/              # 页面组件
│   ├── router/             # 路由配置
│   ├── stores/             # 状态管理
│   ├── styles/             # 核心样式表
│   ├── utils/              # 前端工具函数
│   ├── views/              # 视图组件
│   ├── App.vue             # 根组件
│   └── main.ts             # 应用入口
│
├── src-tauri/              # 🦀 Rust 后端源代码
│   ├── src/                # Rust 代码主目录
│   │   ├── api/            # API 路由和处理函数
│   │   │   ├── minecraft/  # Minecraft 相关 API
│   │   │   ├── mods/       # 模组相关 API
│   │   │   └── system/     # 系统相关 API
│   │   ├── models/         # 数据模型和结构体
│   │   ├── services/       # 业务逻辑服务
│   │   │   ├── download/   # 下载服务
│   │   │   ├── launcher/   # 启动器服务
│   │   │   └── auth/       # 认证服务
│   │   ├── utils/          # 工具函数
│   │   ├── config/         # 配置管理
│   │   ├── db/             # 数据库交互
│   │   ├── error/          # 错误处理
│   │   ├── middleware/     # 中间件
│   │   ├── lib.rs          # 库入口
│   │   └── main.rs         # 应用入口
│   ├── Cargo.toml          # Rust 项目配置
│   └── tauri.conf.json     # Tauri 配置文件
│
├── public/                 # 🌐 静态资源
│
├── docs/                   # 📚 项目文档
│   └── Coding Standards/   # 编码规范
│
├── .env.development        # 环境变量
├── package.json            # Node 项目配置
└── README.md