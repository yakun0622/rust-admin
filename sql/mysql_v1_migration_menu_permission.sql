-- sys_menu 新增标准权限字段 permission（幂等）
SET @db_name = DATABASE();

SET @column_exists = (
  SELECT COUNT(1)
  FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = @db_name
    AND TABLE_NAME = 'sys_menu'
    AND COLUMN_NAME = 'permission'
);

SET @ddl_add_column = IF(
  @column_exists = 0,
  "ALTER TABLE sys_menu ADD COLUMN permission VARCHAR(100) NULL COMMENT '权限标识（标准字段）' AFTER perms",
  "SELECT 'sys_menu.permission already exists' AS message"
);

PREPARE stmt FROM @ddl_add_column;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @idx_exists = (
  SELECT COUNT(1)
  FROM information_schema.STATISTICS
  WHERE TABLE_SCHEMA = @db_name
    AND TABLE_NAME = 'sys_menu'
    AND INDEX_NAME = 'idx_sys_menu_permission'
);

SET @ddl_add_idx = IF(
  @idx_exists = 0,
  "ALTER TABLE sys_menu ADD INDEX idx_sys_menu_permission (permission)",
  "SELECT 'idx_sys_menu_permission already exists' AS message"
);

PREPARE stmt FROM @ddl_add_idx;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 历史数据回填：优先把 perms 同步到 permission
UPDATE sys_menu
SET permission = perms
WHERE (permission IS NULL OR permission = '')
  AND perms IS NOT NULL
  AND perms <> '';

-- 兼容双写：补齐 perms（避免旧代码读取不到）
UPDATE sys_menu
SET perms = permission
WHERE (perms IS NULL OR perms = '')
  AND permission IS NOT NULL
  AND permission <> '';

-- 补齐按钮级权限种子（幂等）
INSERT IGNORE INTO sys_menu
(`id`, `parent_id`, `menu_type`, `menu_name`, `route_name`, `route_path`, `component_path`, `perms`, `permission`, `icon`, `order_num`, `is_external`, `is_cache`, `is_visible`, `status`, `remark`, `created_by`)
VALUES
  (211, 201, 3, '用户新增', NULL, NULL, NULL, 'system:user:create', 'system:user:create', NULL, 1, 0, 0, 0, 1, '用户新增按钮', 1),
  (212, 201, 3, '用户编辑', NULL, NULL, NULL, 'system:user:update', 'system:user:update', NULL, 2, 0, 0, 0, 1, '用户编辑按钮', 1),
  (213, 201, 3, '用户删除', NULL, NULL, NULL, 'system:user:delete', 'system:user:delete', NULL, 3, 0, 0, 0, 1, '用户删除按钮', 1),
  (221, 202, 3, '角色新增', NULL, NULL, NULL, 'system:role:create', 'system:role:create', NULL, 1, 0, 0, 0, 1, '角色新增按钮', 1),
  (222, 202, 3, '角色编辑', NULL, NULL, NULL, 'system:role:update', 'system:role:update', NULL, 2, 0, 0, 0, 1, '角色编辑按钮', 1),
  (223, 202, 3, '角色删除', NULL, NULL, NULL, 'system:role:delete', 'system:role:delete', NULL, 3, 0, 0, 0, 1, '角色删除按钮', 1),
  (231, 203, 3, '菜单新增', NULL, NULL, NULL, 'system:menu:create', 'system:menu:create', NULL, 1, 0, 0, 0, 1, '菜单新增按钮', 1),
  (232, 203, 3, '菜单编辑', NULL, NULL, NULL, 'system:menu:update', 'system:menu:update', NULL, 2, 0, 0, 0, 1, '菜单编辑按钮', 1),
  (233, 203, 3, '菜单删除', NULL, NULL, NULL, 'system:menu:delete', 'system:menu:delete', NULL, 3, 0, 0, 0, 1, '菜单删除按钮', 1),
  (241, 204, 3, '部门新增', NULL, NULL, NULL, 'system:dept:create', 'system:dept:create', NULL, 1, 0, 0, 0, 1, '部门新增按钮', 1),
  (242, 204, 3, '部门编辑', NULL, NULL, NULL, 'system:dept:update', 'system:dept:update', NULL, 2, 0, 0, 0, 1, '部门编辑按钮', 1),
  (243, 204, 3, '部门删除', NULL, NULL, NULL, 'system:dept:delete', 'system:dept:delete', NULL, 3, 0, 0, 0, 1, '部门删除按钮', 1),
  (251, 205, 3, '岗位新增', NULL, NULL, NULL, 'system:post:create', 'system:post:create', NULL, 1, 0, 0, 0, 1, '岗位新增按钮', 1),
  (252, 205, 3, '岗位编辑', NULL, NULL, NULL, 'system:post:update', 'system:post:update', NULL, 2, 0, 0, 0, 1, '岗位编辑按钮', 1),
  (253, 205, 3, '岗位删除', NULL, NULL, NULL, 'system:post:delete', 'system:post:delete', NULL, 3, 0, 0, 0, 1, '岗位删除按钮', 1),
  (261, 206, 3, '字典新增', NULL, NULL, NULL, 'system:dict:create', 'system:dict:create', NULL, 1, 0, 0, 0, 1, '字典新增按钮', 1),
  (262, 206, 3, '字典编辑', NULL, NULL, NULL, 'system:dict:update', 'system:dict:update', NULL, 2, 0, 0, 0, 1, '字典编辑按钮', 1),
  (263, 206, 3, '字典删除', NULL, NULL, NULL, 'system:dict:delete', 'system:dict:delete', NULL, 3, 0, 0, 0, 1, '字典删除按钮', 1),
  (271, 207, 3, '参数新增', NULL, NULL, NULL, 'system:config:create', 'system:config:create', NULL, 1, 0, 0, 0, 1, '参数新增按钮', 1),
  (272, 207, 3, '参数编辑', NULL, NULL, NULL, 'system:config:update', 'system:config:update', NULL, 2, 0, 0, 0, 1, '参数编辑按钮', 1),
  (273, 207, 3, '参数删除', NULL, NULL, NULL, 'system:config:delete', 'system:config:delete', NULL, 3, 0, 0, 0, 1, '参数删除按钮', 1),
  (281, 208, 3, '公告新增', NULL, NULL, NULL, 'system:notice:create', 'system:notice:create', NULL, 1, 0, 0, 0, 1, '公告新增按钮', 1),
  (282, 208, 3, '公告编辑', NULL, NULL, NULL, 'system:notice:update', 'system:notice:update', NULL, 2, 0, 0, 0, 1, '公告编辑按钮', 1),
  (283, 208, 3, '公告删除', NULL, NULL, NULL, 'system:notice:delete', 'system:notice:delete', NULL, 3, 0, 0, 0, 1, '公告删除按钮', 1),
  (311, 301, 3, '操作日志清理', NULL, NULL, NULL, 'log:oper:delete', 'log:oper:delete', NULL, 1, 0, 0, 0, 1, '操作日志清理按钮', 1),
  (312, 302, 3, '登录日志清理', NULL, NULL, NULL, 'log:login:delete', 'log:login:delete', NULL, 1, 0, 0, 0, 1, '登录日志清理按钮', 1),
  (421, 401, 3, '在线用户强退', NULL, NULL, NULL, 'monitor:online:kickout', 'monitor:online:kickout', NULL, 1, 0, 0, 0, 1, '在线用户强退按钮', 1),
  (422, 402, 3, '任务新增', NULL, NULL, NULL, 'monitor:job:create', 'monitor:job:create', NULL, 1, 0, 0, 0, 1, '任务新增按钮', 1),
  (423, 402, 3, '任务编辑', NULL, NULL, NULL, 'monitor:job:update', 'monitor:job:update', NULL, 2, 0, 0, 0, 1, '任务编辑按钮', 1),
  (424, 402, 3, '任务删除', NULL, NULL, NULL, 'monitor:job:delete', 'monitor:job:delete', NULL, 3, 0, 0, 0, 1, '任务删除按钮', 1),
  (425, 402, 3, '任务执行', NULL, NULL, NULL, 'monitor:job:run', 'monitor:job:run', NULL, 4, 0, 0, 0, 1, '任务立即执行按钮', 1),
  (426, 402, 3, '任务暂停', NULL, NULL, NULL, 'monitor:job:pause', 'monitor:job:pause', NULL, 5, 0, 0, 0, 1, '任务暂停按钮', 1),
  (427, 402, 3, '任务恢复', NULL, NULL, NULL, 'monitor:job:resume', 'monitor:job:resume', NULL, 6, 0, 0, 0, 1, '任务恢复按钮', 1),
  (451, 405, 3, '缓存搜索', NULL, NULL, NULL, 'monitor:cache:search', 'monitor:cache:search', NULL, 1, 0, 0, 0, 1, '缓存搜索按钮', 1);

INSERT IGNORE INTO sys_role_menu
(`id`, `role_id`, `menu_id`, `created_by`)
VALUES
  (1021, 1, 211, 1),
  (1022, 1, 212, 1),
  (1023, 1, 213, 1),
  (1024, 1, 221, 1),
  (1025, 1, 222, 1),
  (1026, 1, 223, 1),
  (1027, 1, 231, 1),
  (1028, 1, 232, 1),
  (1029, 1, 233, 1),
  (1030, 1, 241, 1),
  (1031, 1, 242, 1),
  (1032, 1, 243, 1),
  (1033, 1, 251, 1),
  (1034, 1, 252, 1),
  (1035, 1, 253, 1),
  (1036, 1, 261, 1),
  (1037, 1, 262, 1),
  (1038, 1, 263, 1),
  (1039, 1, 271, 1),
  (1040, 1, 272, 1),
  (1041, 1, 273, 1),
  (1042, 1, 281, 1),
  (1043, 1, 282, 1),
  (1044, 1, 283, 1),
  (1045, 1, 311, 1),
  (1046, 1, 312, 1),
  (1047, 1, 421, 1),
  (1048, 1, 422, 1),
  (1049, 1, 423, 1),
  (1050, 1, 424, 1),
  (1051, 1, 425, 1),
  (1052, 1, 426, 1),
  (1053, 1, 427, 1),
  (1054, 1, 451, 1);
