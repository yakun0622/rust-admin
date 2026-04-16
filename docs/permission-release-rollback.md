# 菜单到按钮级权限发布与回滚说明

> 适用范围：`sys_menu.permission` 字段、`/api/system/auth/profile`、后端 `ensure_permission` 拦截、前端动态菜单与按钮权限显隐。

## 1. 发布前检查

1. 代码版本包含：
   - 后端权限聚合与接口拦截改造。
   - 前端 `AuthSession` 与动态菜单/按钮权限控制。
   - 回归脚本 `scripts/regression/permission_api.sh`。
2. 数据库已备份（至少备份 `sys_menu/sys_role_menu/sys_user_role`）。
3. 目标环境已确认 `sys_menu` 存在 `permission` 字段（或可执行迁移脚本补齐）。

## 2. 发布步骤（推荐顺序）

1. 执行数据库迁移：
```bash
mysql -uroot -p rust_admin < sql/mysql_v1_migration_menu_permission.sql
```
2. 发布后端服务（`admin-api`）。
3. 发布前端应用（`admin-web`）。
4. 依次执行验证：
```bash
cd admin-api
cargo check
bash scripts/regression/permission_api.sh

cd ../admin-web
npm run typecheck
npm run build
```

## 3. 发布后验收

1. `admin` 登录后 `GET /api/system/auth/profile` 返回 `permissions` 与 `menus`。
2. 低权限账号访问受控接口返回 `403`。
3. 前端侧边栏与按钮显隐符合角色权限配置。
4. 核心写接口（如用户/角色/菜单新增编辑删除）都被权限拦截保护。

## 4. 回滚策略

## 4.1 应用层回滚（优先）

1. 回滚 `admin-api` 到上一个稳定版本。
2. 回滚 `admin-web` 到上一个稳定版本。
3. 保留数据库 `permission` 字段不删（向后兼容，不影响旧代码读取 `perms`）。

## 4.2 数据层应急回滚（仅必要时）

1. 若新版本对 `permission` 的写入导致异常，可先把 `permission` 回填到 `perms`：
```sql
UPDATE sys_menu
SET perms = permission
WHERE (perms IS NULL OR perms = '')
  AND permission IS NOT NULL
  AND permission <> '';
```
2. 回滚后端应用到旧版本读取逻辑。
3. 不建议直接删除 `permission` 字段，避免影响后续再次发布。

## 5. 已知边界

1. 当前仅支持 MySQL。
2. 权限缓存失效策略需与角色授权变更流程联动，避免权限延迟生效。
3. 前端显隐是体验层，真正安全边界在后端 `ensure_permission`。
