# admin-web

React + TypeScript + Ant Design 管理后台前端工程，按 `app / core / modules / shared` 分层组织。

## 目录说明

```text
admin-web/
├─ public/                         # 静态资源
├─ src/
│  ├─ main.tsx                     # 前端入口
│  ├─ app/                         # 应用壳层
│  │  ├─ router/                   # 路由聚合
│  │  ├─ providers/                # 全局 Provider
│  │  └─ layouts/                  # 布局组件
│  ├─ core/                        # 全局核心能力
│  │  ├─ request/                  # axios 封装
│  │  ├─ auth/                     # token 管理
│  │  ├─ permission/               # 权限工具
│  │  └─ config/                   # 全局配置
│  ├─ modules/                     # 业务模块
│  │  ├─ auth/
│  │  ├─ dashboard/
│  │  ├─ system/
│  │  ├─ log/
│  │  ├─ monitor/
│  │  └─ ai/
│  ├─ shared/                      # 共享组件/hooks/types/utils
│  └─ styles/                      # 全局样式
├─ vite.config.ts                  # Vite 配置（含 /api 代理）
└─ package.json
```

## 环境要求

1. Node.js `>=16.14`
2. npm `>=8`

## 配置说明

默认读取 `.env` 文件，当前示例文件：

```env
VITE_API_BASE_URL=/api
```

开发模式下，`/api` 会被代理到 `http://127.0.0.1:8080`（见 `vite.config.ts`）。

## 运行步骤

1. 安装依赖：
```bash
cd admin-web
npm install
```
2. 启动开发服务：
```bash
npm run dev
```
3. 访问：
```text
http://127.0.0.1:5173
```

## 常用命令

```bash
cd admin-web
npm run dev
npm run typecheck
npm run build
npm run preview
```

## 联调说明

1. 请先启动后端 `admin-api`（默认 `8080`）。
2. 登录页当前默认账号密码：
```text
admin / admin123456
```
3. 登录后会访问：
   1. `POST /api/auth/login`
   2. `GET /api/dashboard/overview`
4. 前端已启用 `401` 自动处理：Token 失效会清理本地令牌并跳转登录页。

## 当前页面能力（MVP）

1. 首页看板：Mock 数据块与趋势展示。
2. 系统管理：8 个子模块统一 CRUD（查询/新增/编辑/删除）。
3. 日志管理：操作日志、登录日志查询。
4. 系统监控：
   1. 在线用户查询
   2. 定时任务管理（新增/编辑/删除/执行一次/暂停/恢复）
   3. 数据监控、服务监控展示
   4. 缓存监控搜索（只读）
   5. 缓存列表统计
5. AI 对话：会话与消息的后端 Mock 交互。
