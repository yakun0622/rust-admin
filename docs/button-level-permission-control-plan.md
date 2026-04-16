# 菜单到按钮级权限控制实施方案（阶段任务版）

> 日期：2026-04-16  
> 目标：实现“菜单可见 + 按钮可操作”的统一权限控制，并新增 `permission` 字段作为标准权限标识。

---

## 1. 背景与现状

当前系统已经具备以下基础：

1. `sys_menu.menu_type` 已支持 `1目录 / 2菜单 / 3按钮`。
2. `sys_role_menu(role_id, menu_id)` 已存在，可用于角色菜单关联。
3. 前端已有 `hasPermission(userPerms, requiredPerm)` 工具，但尚未接通后端真实权限数据。
4. 后端当前仅做“登录态校验”（JWT），未做接口级权限拦截。

当前主要问题：

1. 业务对象只使用了菜单的基础字段，`sys_menu` 的权限字段未贯通到 DTO/Model/VO/API。
2. API 没有“按权限码拦截”能力。
3. 前端菜单与按钮未基于真实权限动态控制（当前仍以静态配置为主）。

---

## 2. 目标范围

## 2.1 功能目标

1. 菜单控制到按钮级别（`menu_type=3` 用于按钮权限）。
2. 新增并标准化 `permission` 字段。
3. 后端接口支持权限拦截（至少覆盖 `system/monitor/log` 的写操作接口）。
4. 前端支持：
   - 菜单动态过滤（页面权限）
   - 按钮显隐与禁用（操作权限）

## 2.2 非目标（本阶段不做）

1. 数据权限（如“仅看本部门数据”）细粒度行级控制。
2. 多租户隔离策略。
3. Casbin/OpenFGA 外部策略引擎接入（后续可扩展）。

---

## 3. 字段设计与兼容策略

## 3.1 字段方案

`sys_menu` 增加标准字段：

1. `permission VARCHAR(100) NULL COMMENT '权限标识（标准字段）'`

说明：

1. 当前表已有 `perms` 字段。为降低一次性改造风险，采用“兼容迁移”：
   - 过渡期双字段并存（`permission` 与 `perms`）
   - 读优先 `permission`，为空时回退 `perms`
   - 写入双写（或仅写 `permission` + 触发器/脚本回填 `perms`）
2. 稳定后可在二期删除 `perms`。

## 3.2 约束建议

1. `menu_type=3`（按钮）时，`permission` 必填。
2. `menu_type=1/2`（目录/菜单）可选 `permission`（用于页面访问控制）。
3. 权限码规范：`模块:资源:动作`，如：
   - `system:user:view`
   - `system:user:create`
   - `system:user:update`
   - `system:user:delete`

---

## 4. 后端实现方案

## 4.1 数据模型贯通

涉及层：

1. `core/model/sys_menu.rs` 增加 `menu_type`、`permission` 等字段。
2. `core/dto/sys_menu_dto.rs` 新增 `menu_type`、`permission` 入参定义与校验规则。
3. `core/vo/sys_menu_vo.rs` 输出 `menu_type`、`permission`。
4. `core/converter/sys_menu_converter.rs` 完成双字段映射与兼容逻辑。
5. `modules/system/repository/sys_menu_repository.rs` 查询/新增/更新改为读写 `permission`（兼容 `perms`）。

## 4.2 登录后权限装载

新增接口（建议）：

1. `GET /api/system/auth/profile`

返回：

1. 当前用户基本信息
2. `permissions: string[]`
3. `menus: MenuTree[]`（仅目录/菜单，按钮不作为侧边栏节点）

权限查询逻辑：

1. 通过 `user_id -> sys_user_role -> sys_role_menu -> sys_menu` 获取权限码集合。
2. 条件过滤：`sys_menu.status=1 AND is_deleted=0`。
3. 权限集合来源：
   - 页面权限：`menu_type in (2)`（可按需要含目录）
   - 按钮权限：`menu_type=3`

## 4.3 接口级权限拦截

新增中间件/提取器（建议命名）：

1. `permission!(state, current_user, "system:user:create")`
2. `admin_log!(state, current_user, "创建用户", 1_i8, async move { ... })`

`admin_log` 参数约定（精简版）：

1. 必填：`name`（接口名称，如“创建用户”）
2. 必填：`business_type`（`1=新增`，`2=修改`，`3=删除`，`4=授权/其他动作`）
3. 可选扩展：`request_params`（使用 6 参宏版本时传入）

处理流程：

1. 先复用现有登录中间件拿到 `CurrentUser`。
2. 读取当前用户权限集合（优先缓存，未命中查库）。
3. 校验是否包含目标权限码或超级权限 `*:*:*`。
4. 不通过返回 `403`。

## 4.4 缓存策略（Redis）

缓存 key 建议：

1. `auth:user:perms:{user_id}`
2. `auth:user:menus:{user_id}`

失效时机：

1. 角色授权变更（`role_menu`）
2. 用户角色变更（`user_role`）
3. 菜单权限字段更新（`sys_menu.permission`）

---

## 5. 前端实现方案

## 5.1 权限状态管理

1. 登录后拉取 `/api/system/auth/profile`。
2. 将 `permissions` 存入全局状态（如 Zustand/Context）。
3. 用现有 `hasPermission()` 实现按钮级判断。

## 5.2 菜单动态化

1. 侧边菜单由后端返回 `menus` 驱动。
2. 目录/菜单展示规则：
   - `menu_type in (1,2)`
   - `is_visible=1`
   - 用户具备对应权限（若配置了 `permission`）

## 5.3 按钮级控制

1. 页面内按钮增加 `requiredPerm` 配置，例如：
   - 新增按钮：`system:user:create`
   - 编辑按钮：`system:user:update`
   - 删除按钮：`system:user:delete`
2. 无权限时隐藏（或置灰不可点击，按产品策略）。

---

## 6. 分阶段任务

## 阶段 A：表结构与种子数据 ✅

任务：

1. ✅ 新增 `sys_menu.permission` 字段迁移脚本。
2. ✅ 将已有 `perms` 回填到 `permission`。
3. ✅ 为按钮类型菜单补齐标准权限码种子数据。

DoD：

1. ✅ 新老环境执行迁移后，`permission` 有效可读。
2. ✅ 核心菜单/按钮都有规范权限码。

## 阶段 B：后端菜单对象与接口改造 ✅

任务：

1. ✅ `sys_menu` 的 DTO/Model/VO/Converter/Repository 全量增加 `permission`。
2. ✅ 菜单新增/编辑接口支持 `menu_type` + `permission` 维护。
3. ✅ 菜单查询新增树接口 `GET /api/system/menu/tree`，并保留按钮节点（`menu_type=3`）。

DoD：

1. ✅ `menu` CRUD 全链路支持 `permission` 字段。
2. ✅ 按钮菜单可正常创建、查询、编辑。

## 阶段 C：用户权限聚合与拦截 ✅

任务：

1. ✅ 新增 `GET /api/system/auth/profile`（返回 `permissions + menus`）。
2. ✅ 实现权限校验函数 `ensure_permission`（统一返回 403）。
3. ✅ 给关键写接口加权限校验（覆盖 `system` 核心模块）。

DoD：

1. ✅ 无权限访问写接口返回 `403`。
2. ✅ 超级权限 `*:*:*` 可放行。

## 阶段 D：前端菜单与按钮联动 ✅

任务：

1. ✅ 登录后加载权限并落地前端状态（`AuthSession` 全局上下文）。
2. ✅ 侧边菜单切换为后端动态数据（基于 `/api/system/auth/profile` 返回菜单树）。
3. ✅ CRUD 页面按钮接入权限码控制（`view/create/update/delete`）。

DoD：

1. ✅ 不同角色看到的菜单与按钮明显不同。
2. ✅ 前端隐藏与后端拦截双重生效。

## 阶段 E：回归与文档封版（进行中）

任务：

1. ✅ 增加权限场景回归脚本（有权/无权/超管）。
2. ✅ 更新 `README` 的权限章节与权限码规范。
3. ✅ 输出上线与回滚说明。

DoD：

1. ⏳ `cargo check`、前端构建、核心权限回归通过。
2. ✅ 文档可直接指导新业务接入权限码。

当前状态：

1. ✅ 已完成 `cargo check`。
2. ✅ 已完成前端 `npm run typecheck && npm run build`。
3. ⏳ `permission_api.sh` 待在可访问运行中的 API 环境执行。

---

## 7. 验收清单（阶段总验收）

1. 菜单管理页可维护 `menu_type=3` 按钮节点和 `permission`。
2. 登录后能拿到当前用户权限集合。
3. 前端按钮显隐正确。
4. 后端写接口被权限中间件正确拦截。
5. 角色授权变更后，权限缓存能及时失效。

---

## 8. 风险与控制

1. 风险：`perms` 与 `permission` 双字段不一致。  
控制：迁移期统一读策略 + 定时比对脚本 + 上线前全量校验。

2. 风险：前端只做显隐，后端未拦截导致越权。  
控制：所有敏感写接口必须加 `permission`，并在写接口补充 `admin_log`。

3. 风险：权限缓存未失效导致权限延迟生效。  
控制：授权变更流程中强制删除对应用户缓存 key。

---

## 9. 任务看板

- [x] 阶段 A：表结构与种子数据 ✅
- [x] 阶段 B：后端菜单对象与接口改造 ✅
- [x] 阶段 C：用户权限聚合与拦截 ✅
- [x] 阶段 D：前端菜单与按钮联动 ✅
- [ ] 阶段 E：回归与文档封版（进行中）
