BEGIN;

INSERT INTO sys_dept
(id, parent_id, ancestors, dept_name, order_num, leader, phone, status, remark, created_by)
VALUES
  (100, 0, '0', '平台管理中心', 1, '系统管理员', '13800000000', 1, '系统初始化部门', 1),
  (101, 100, '0,100', '系统运维部', 1, '运维负责人', '13800000001', 1, '系统初始化二级部门', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_post
(id, post_code, post_name, post_sort, status, remark, created_by)
VALUES
  (100, 'SYS_ADMIN', '系统管理员', 1, 1, '系统初始化岗位', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_role
(id, role_name, role_key, role_sort, data_scope, status, is_system, remark, created_by)
VALUES
  (1, '超级管理员', 'super_admin', 1, 1, 1, 1, '系统内置超级管理员角色', 1),
  (2, '系统管理员', 'system_admin', 2, 2, 1, 1, '系统内置系统管理员角色', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_user
(id, username, nickname, password_hash, phone, email, status, dept_id, remark, created_by)
VALUES
  (1, 'admin', '超级管理员', '$2b$10$jh6uvsoSAuxAfUYOc5ckkecacY3x2zPL0GuvlX38JCpRHM2OtoByi', '13800000000', 'admin@example.com', 1, 100, '系统初始化管理员', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_user_role
(id, user_id, role_id, created_by)
VALUES
  (1, 1, 1, 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_user_post
(id, user_id, post_id, created_by)
VALUES
  (1, 1, 100, 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_menu
(id, parent_id, menu_type, menu_name, route_name, route_path, component_path, perms, icon, order_num, is_external, is_cache, is_visible, status, remark, created_by)
VALUES
  (100, 0, 2, 'AI对话', 'AiChat', '/ai/chat', 'pages/ai/chat/index', 'ai:chat:view', 'robot', 1, 0, 0, 1, 1, 'AI对话菜单', 1),
  (200, 0, 1, '系统管理', 'System', '/system', NULL, NULL, 'setting', 2, 0, 0, 1, 1, '系统管理目录', 1),
  (201, 200, 2, '用户管理', 'SystemUser', '/system/user', 'pages/system/user/index', 'system:user:view', 'user', 1, 0, 0, 1, 1, '用户管理菜单', 1),
  (202, 200, 2, '角色管理', 'SystemRole', '/system/role', 'pages/system/role/index', 'system:role:view', 'team', 2, 0, 0, 1, 1, '角色管理菜单', 1),
  (203, 200, 2, '菜单管理', 'SystemMenu', '/system/menu', 'pages/system/menu/index', 'system:menu:view', 'menu', 3, 0, 0, 1, 1, '菜单管理菜单', 1),
  (204, 200, 2, '部门管理', 'SystemDept', '/system/dept', 'pages/system/dept/index', 'system:dept:view', 'apartment', 4, 0, 0, 1, 1, '部门管理菜单', 1),
  (205, 200, 2, '岗位管理', 'SystemPost', '/system/post', 'pages/system/post/index', 'system:post:view', 'idcard', 5, 0, 0, 1, 1, '岗位管理菜单', 1),
  (206, 200, 2, '字典管理', 'SystemDict', '/system/dict', 'pages/system/dict/index', 'system:dict:view', 'book', 6, 0, 0, 1, 1, '字典管理菜单', 1),
  (207, 200, 2, '参数设置', 'SystemConfig', '/system/config', 'pages/system/config/index', 'system:config:view', 'tool', 7, 0, 0, 1, 1, '参数设置菜单', 1),
  (208, 200, 2, '通知公告', 'SystemNotice', '/system/notice', 'pages/system/notice/index', 'system:notice:view', 'notification', 8, 0, 0, 1, 1, '通知公告菜单', 1),
  (300, 0, 1, '日志管理', 'Log', '/log', NULL, NULL, 'file-text', 3, 0, 0, 1, 1, '日志管理目录', 1),
  (301, 300, 2, '操作日志', 'LogOper', '/log/oper', 'pages/log/oper/index', 'log:oper:view', 'profile', 1, 0, 0, 1, 1, '操作日志菜单', 1),
  (302, 300, 2, '登录日志', 'LogLogin', '/log/login', 'pages/log/login/index', 'log:login:view', 'login', 2, 0, 0, 1, 1, '登录日志菜单', 1),
  (400, 0, 1, '系统监控', 'Monitor', '/monitor', NULL, NULL, 'monitor', 4, 0, 0, 1, 1, '系统监控目录', 1),
  (401, 400, 2, '在线用户', 'MonitorOnline', '/monitor/online', 'pages/monitor/online/index', 'monitor:online:view', 'usergroup-add', 1, 0, 0, 1, 1, '在线用户菜单', 1),
  (402, 400, 2, '定时任务', 'MonitorJob', '/monitor/job', 'pages/monitor/job/index', 'monitor:job:view', 'schedule', 2, 0, 0, 1, 1, '定时任务菜单', 1),
  (403, 400, 2, '数据监控', 'MonitorDatasource', '/monitor/datasource', 'pages/monitor/datasource/index', 'monitor:datasource:view', 'database', 3, 0, 0, 1, 1, '数据监控菜单', 1),
  (404, 400, 2, '服务监控', 'MonitorServer', '/monitor/server', 'pages/monitor/server/index', 'monitor:server:view', 'cloud-server', 4, 0, 0, 1, 1, '服务监控菜单', 1),
  (405, 400, 2, '缓存监控', 'MonitorCache', '/monitor/cache', 'pages/monitor/cache/index', 'monitor:cache:view', 'hdd', 5, 0, 0, 1, 1, '缓存监控菜单', 1),
  (406, 400, 2, '缓存列表', 'MonitorCacheList', '/monitor/cache-list', 'pages/monitor/cache-list/index', 'monitor:cache-list:view', 'bars', 6, 0, 0, 1, 1, '缓存列表菜单', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_role_menu
(id, role_id, menu_id, created_by)
VALUES
  (1001, 1, 100, 1),
  (1002, 1, 200, 1),
  (1003, 1, 201, 1),
  (1004, 1, 202, 1),
  (1005, 1, 203, 1),
  (1006, 1, 204, 1),
  (1007, 1, 205, 1),
  (1008, 1, 206, 1),
  (1009, 1, 207, 1),
  (1010, 1, 208, 1),
  (1011, 1, 300, 1),
  (1012, 1, 301, 1),
  (1013, 1, 302, 1),
  (1014, 1, 400, 1),
  (1015, 1, 401, 1),
  (1016, 1, 402, 1),
  (1017, 1, 403, 1),
  (1018, 1, 404, 1),
  (1019, 1, 405, 1),
  (1020, 1, 406, 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_dict_type
(id, dict_name, dict_type, status, remark, created_by)
VALUES
  (100, '通用状态', 'sys_common_status', 1, '系统初始化字典', 1),
  (101, '公告类型', 'sys_notice_type', 1, '系统初始化字典', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_dict_data
(id, dict_type_id, label, value, tag_type, is_default, sort, status, remark, created_by)
VALUES
  (1001, 100, '启用', '1', 'success', 1, 1, 1, '启用状态', 1),
  (1002, 100, '停用', '0', 'default', 0, 2, 1, '停用状态', 1),
  (1011, 101, '通知', '1', 'info', 1, 1, 1, '通知类型', 1),
  (1012, 101, '公告', '2', 'warning', 0, 2, 1, '公告类型', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_config
(id, config_name, config_key, config_value, config_type, status, remark, created_by)
VALUES
  (100, '系统名称', 'sys.app.name', 'Rust Admin', 1, 1, '系统初始化参数', 1),
  (101, '首页看板数据模式', 'sys.dashboard.mode', 'mock', 1, 1, '首期看板使用Mock数据', 1),
  (102, 'AI对话模式', 'sys.ai.mode', 'mock', 1, 1, '首期AI对话仅Mock交互', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_notice
(id, title, notice_type, summary, content, status, published_by, published_at, version_no, remark, created_by)
VALUES
  (100, '系统初始化完成', 2, '后台管理系统基础数据初始化公告', '系统基础数据已初始化，可使用 admin 账号进入后台进行后续配置。', 1, 1, CURRENT_TIMESTAMP(3), 1, '初始化公告', 1)
ON CONFLICT (id) DO NOTHING;

SELECT setval(pg_get_serial_sequence('sys_dept', 'id'), COALESCE((SELECT MAX(id) FROM sys_dept), 1), true);
SELECT setval(pg_get_serial_sequence('sys_post', 'id'), COALESCE((SELECT MAX(id) FROM sys_post), 1), true);
SELECT setval(pg_get_serial_sequence('sys_role', 'id'), COALESCE((SELECT MAX(id) FROM sys_role), 1), true);
SELECT setval(pg_get_serial_sequence('sys_menu', 'id'), COALESCE((SELECT MAX(id) FROM sys_menu), 1), true);
SELECT setval(pg_get_serial_sequence('sys_user', 'id'), COALESCE((SELECT MAX(id) FROM sys_user), 1), true);
SELECT setval(pg_get_serial_sequence('sys_user_role', 'id'), COALESCE((SELECT MAX(id) FROM sys_user_role), 1), true);
SELECT setval(pg_get_serial_sequence('sys_user_post', 'id'), COALESCE((SELECT MAX(id) FROM sys_user_post), 1), true);
SELECT setval(pg_get_serial_sequence('sys_role_menu', 'id'), COALESCE((SELECT MAX(id) FROM sys_role_menu), 1), true);
SELECT setval(pg_get_serial_sequence('sys_dict_type', 'id'), COALESCE((SELECT MAX(id) FROM sys_dict_type), 1), true);
SELECT setval(pg_get_serial_sequence('sys_dict_data', 'id'), COALESCE((SELECT MAX(id) FROM sys_dict_data), 1), true);
SELECT setval(pg_get_serial_sequence('sys_config', 'id'), COALESCE((SELECT MAX(id) FROM sys_config), 1), true);
SELECT setval(pg_get_serial_sequence('sys_notice', 'id'), COALESCE((SELECT MAX(id) FROM sys_notice), 1), true);

COMMIT;
