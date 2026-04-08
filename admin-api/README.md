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
│  └─ modules/                     # 业务模块（auth/dashboard/system/log/monitor/ai）
└─ tests/                          # 测试目录
```

## 配置说明

当前使用纯 `toml` 多环境配置。

1. `config/default.toml`：默认配置。
2. `config/dev.toml`：开发环境覆盖（MySQL）。
3. `config/pg.toml`：开发环境覆盖（PostgreSQL）。
4. `config/prod.toml`：生产环境覆盖。
5. `.env` 仅保留 `APP_ENV` 用于选择环境。

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
driver = "mysql" # mysql | postgres
url = "mysql://root:123456@127.0.0.1:3306/rust_admin?charset=utf8mb4"
max_connections = 20
min_connections = 5
acquire_timeout_secs = 5
```

PostgreSQL 示例：

```toml
[database]
driver = "postgres"
url = "postgres://postgres:123456@127.0.0.1:5432/rust_admin"
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
3. 初始化数据库（PostgreSQL）：
```bash
createdb rust_admin
psql -d rust_admin -f ../sql/postgres_v1_schema.sql
psql -d rust_admin -f ../sql/postgres_v1_seed.sql
```
4. 选择环境并启动（MySQL 用 `APP_ENV=dev`，PostgreSQL 可用 `APP_ENV=pg`）：
```bash
cd admin-api
APP_ENV=dev cargo run
```
5. 验证服务：
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
2. `POST /api/auth/login`：登录（MySQL/PostgreSQL 用户仓储 + bcrypt 密码校验 + JWT 签发）。
3. `GET /api/dashboard/overview`：看板数据（mock）。
4. `GET/POST/PUT/DELETE /api/system/:resource`：系统管理 8 类资源 CRUD（已接 MySQL/PostgreSQL 持久化，user/role/menu/dept/post/dict/config/notice）。
5. `GET /api/log/oper`：操作日志查询（MySQL/PostgreSQL，关键字检索）。
6. `GET /api/log/login`：登录日志查询（MySQL/PostgreSQL，关键字检索）。
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

1. 除 `POST /api/auth/login` 与 `GET /health` 外，`/api/**` 默认要求 `Authorization: Bearer <token>`。
2. Token 无效或过期会返回 `401`。

认证审计说明：

1. 登录成功与失败会写入 `sys_login_log`（含用户名、类型、状态、IP、消息）。

## PostgreSQL 阶段说明

当前已完成阶段 A+B+C 的代码落地：包含 PostgreSQL repository、schema/seed 脚本与工厂装配。  
下一步是阶段 D 联调与回归（双库实测、回归清单、发布说明）。

## 阶段 D 工具

1. 双库回归清单：`../docs/postgresql-regression-checklist.md`
2. Smoke 脚本：
```bash
bash scripts/regression/smoke_api.sh
```
3. 发布说明草稿：`../docs/postgresql-compatibility-release-notes.md`
4. 实测结果模板：`../docs/postgresql-validation-report-template.md`
5. 问题追踪表：`../docs/postgresql-issue-tracker.md`
