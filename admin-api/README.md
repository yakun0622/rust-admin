# admin-api

Rust 后端管理 API 服务，采用分层结构 `api -> service -> repository`，并通过 `core` 提供底层公共封装。

## 目录说明

```text
admin-api/
├─ config/                         # 多环境配置（default/dev/prod）
├─ src/
│  ├─ main.rs                      # 程序入口
│  ├─ app/                         # 应用装配层
│  │  ├─ bootstrap/                # 启动流程
│  │  ├─ routes/                   # 顶层路由
│  │  └─ state/                    # 全局状态（配置、Database、Redis、服务实例）
│  ├─ core/                        # 底层与跨模块公共层
│  │  ├─ config/                   # 配置结构与加载逻辑
│  │  ├─ enums/                    # 枚举定义
│  │  ├─ errors/                   # 统一错误
│  │  ├─ response/                 # 统一响应模型
│  │  ├─ dto/vo/model/converter/   # 共享对象与转换
│  │  └─ ...                       # logging/utils/pagination
│  ├─ middleware/                  # 中间件占位
│  └─ modules/                     # 业务模块（dashboard/system/monitor/ai）
└─ tests/                          # 测试目录
```

## 配置说明

当前使用纯 `toml` 多环境配置。

1. `config/default.toml`：默认配置。
2. `config/dev.toml`：开发环境覆盖（MySQL）。
3. `config/prod.toml`：生产环境覆盖。
4. `.env` 仅保留 `APP_ENV` 用于选择环境。

配置加载顺序：

1. `default.toml`
2. `{APP_ENV}.toml`

示例 `.env`：

```env
APP_ENV=dev
```

`security` 配置项（生产必配）：

```toml
[security]
jwt_secret = "replace-with-a-strong-secret"
jwt_expires_secs = 7200
```

`database` 配置项（阶段 A）：

```toml
[database]
driver = "mysql"
url = "mysql://root:123456@127.0.0.1:3306/rust_admin?charset=utf8mb4"
max_connections = 20
min_connections = 5
acquire_timeout_secs = 5
```

说明：已兼容读取旧版 `[mysql]` 配置段，便于平滑升级。

## 运行步骤

1. 准备依赖服务：启动数据库和 Redis。
2. 初始化数据库（MySQL）：
```bash
mysql -uroot -p < ../sql/mysql_v1_schema.sql
mysql -uroot -p < ../sql/mysql_v1_seed.sql
```
说明：`mysql_v1_seed.sql` 已更新默认管理员 `admin` 的 bcrypt 密码哈希（明文密码 `admin123456`）。
3. 选择环境并启动：
```bash
cd admin-api
APP_ENV=dev cargo run
```
4. 验证服务：
```bash
curl http://127.0.0.1:8080/health
```

`/health` 会返回 `database` 和 `redis` 连接状态。

## 常用命令

```bash
cd admin-api
cargo check
cargo run
```

## 当前已实现接口

1. `GET /health`：服务与依赖健康检查。
2. `POST /api/system/auth/login`：登录（`system/sys_auth`，MySQL）。
3. `GET /api/dashboard/overview`：看板数据（mock）。
4. 系统管理 8 类资源 CRUD（`user/role/menu/dept/post/dict/config/notice`）：
   - `GET/POST /api/system/{resource}`
   - `PUT/DELETE /api/system/{resource}/{id}`
   - 当前按 `api -> service -> repository` 独立资源链路实现，`system` 模块仅启用 MySQL。
5. `GET /api/log/oper`：操作日志查询（由 `system/sys_log` 提供，MySQL，关键字检索）。
6. `GET /api/log/login`：登录日志查询（由 `system/sys_log` 提供，MySQL，关键字检索）。
7. `GET /api/monitor/online`：在线用户查询。
8. `GET/POST/PUT/DELETE /api/monitor/job`：定时任务管理。
9. `POST /api/monitor/job/:id/run|pause|resume`：任务执行与状态控制（内置调度器）。
10. `GET /api/monitor/datasource`：数据源监控。
11. `GET /api/monitor/server`：服务监控。
12. `GET /api/monitor/cache`：缓存搜索（只读）。
13. `GET /api/monitor/cache-list`：缓存命名空间统计。
14. `GET/POST /api/ai/sessions`：AI Mock 会话列表/创建。
15. `GET/POST /api/ai/sessions/:session_id/messages`：AI Mock 消息列表/发送。

鉴权说明：

1. 除 `POST /api/system/auth/login` 与 `GET /health` 外，`/api/**` 默认要求 `Authorization: Bearer <token>`。
2. Token 无效或过期会返回 `401`。

认证审计说明：

1. 登录成功与失败会写入 `sys_login_log`（含用户名、类型、状态、IP、消息）。

## SysUser 模式推广进度

已完成 `A→E` 阶段推广，核心变化如下：

1. `system` 资源统一为 `sys_*` 独立路由与服务实现，不再通过 `resource + Value` 动态透传。
2. `AppState` 已注入 `sys_user/sys_role/sys_menu/sys_dept/sys_post/sys_dict/sys_config/sys_notice` 服务实例。
3. 回归脚本 `scripts/regression/smoke_api.sh` 已覆盖 `system` 8 类资源的 `list/create/update/delete`。

运行 smoke（请先启动服务）：

```bash
bash scripts/regression/smoke_api.sh
```
