SET NAMES utf8mb4;
SET time_zone = '+08:00';

CREATE DATABASE IF NOT EXISTS `rust_admin`
  DEFAULT CHARACTER SET utf8mb4
  DEFAULT COLLATE utf8mb4_general_ci;

USE `rust_admin`;

-- =========================
-- 1) 系统管理
-- =========================

CREATE TABLE IF NOT EXISTS `sys_dept` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '部门ID',
  `parent_id` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '父部门ID',
  `ancestors` VARCHAR(500) NOT NULL DEFAULT '' COMMENT '祖级列表，如0,1,2',
  `dept_name` VARCHAR(64) NOT NULL COMMENT '部门名称',
  `order_num` INT NOT NULL DEFAULT 0 COMMENT '显示顺序',
  `leader` VARCHAR(64) NULL COMMENT '负责人',
  `phone` VARCHAR(20) NULL COMMENT '联系电话',
  `email` VARCHAR(100) NULL COMMENT '邮箱',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_dept_parent_name` (`parent_id`, `dept_name`),
  KEY `idx_sys_dept_parent_status` (`parent_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='部门表';

CREATE TABLE IF NOT EXISTS `sys_post` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '岗位ID',
  `post_code` VARCHAR(64) NOT NULL COMMENT '岗位编码',
  `post_name` VARCHAR(64) NOT NULL COMMENT '岗位名称',
  `post_sort` INT NOT NULL DEFAULT 0 COMMENT '排序',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_post_code` (`post_code`),
  UNIQUE KEY `uk_sys_post_name` (`post_name`),
  KEY `idx_sys_post_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='岗位表';

CREATE TABLE IF NOT EXISTS `sys_role` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `role_name` VARCHAR(64) NOT NULL COMMENT '角色名称',
  `role_key` VARCHAR(64) NOT NULL COMMENT '角色权限标识',
  `role_sort` INT NOT NULL DEFAULT 0 COMMENT '显示顺序',
  `data_scope` TINYINT NOT NULL DEFAULT 5 COMMENT '数据范围：1全部 2本部门及以下 3本部门 4仅本人 5自定义',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `is_system` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否系统内置',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_role_name` (`role_name`),
  UNIQUE KEY `uk_sys_role_key` (`role_key`),
  KEY `idx_sys_role_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='角色表';

CREATE TABLE IF NOT EXISTS `sys_menu` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '菜单ID',
  `parent_id` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '父菜单ID',
  `menu_type` TINYINT NOT NULL COMMENT '菜单类型：1目录 2菜单 3按钮',
  `menu_name` VARCHAR(64) NOT NULL COMMENT '菜单名称',
  `route_name` VARCHAR(64) NULL COMMENT '路由名称',
  `route_path` VARCHAR(128) NULL COMMENT '路由地址',
  `component_path` VARCHAR(255) NULL COMMENT '组件路径',
  `perms` VARCHAR(100) NULL COMMENT '权限标识（兼容旧字段）',
  `permission` VARCHAR(100) NULL COMMENT '权限标识（标准字段）',
  `icon` VARCHAR(100) NULL COMMENT '菜单图标',
  `order_num` INT NOT NULL DEFAULT 0 COMMENT '显示顺序',
  `is_external` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否外链',
  `is_cache` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否缓存',
  `is_visible` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否显示',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_menu_route_path` (`route_path`),
  KEY `idx_sys_menu_parent` (`parent_id`),
  KEY `idx_sys_menu_permission` (`permission`),
  KEY `idx_sys_menu_type_status` (`menu_type`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='菜单表';

CREATE TABLE IF NOT EXISTS `sys_user` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户ID',
  `username` VARCHAR(64) NOT NULL COMMENT '用户名',
  `nickname` VARCHAR(64) NOT NULL COMMENT '昵称',
  `password_hash` VARCHAR(255) NOT NULL COMMENT '密码哈希',
  `phone` VARCHAR(20) NULL COMMENT '手机号',
  `email` VARCHAR(100) NULL COMMENT '邮箱',
  `sex` TINYINT NOT NULL DEFAULT 0 COMMENT '性别：0未知 1男 2女',
  `avatar_url` VARCHAR(255) NULL COMMENT '头像地址',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `dept_id` BIGINT UNSIGNED NULL COMMENT '部门ID',
  `login_ip` VARCHAR(45) NULL COMMENT '最后登录IP',
  `login_at` DATETIME(3) NULL COMMENT '最后登录时间',
  `pwd_updated_at` DATETIME(3) NULL COMMENT '密码更新时间',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_user_username` (`username`),
  KEY `idx_sys_user_phone` (`phone`),
  KEY `idx_sys_user_email` (`email`),
  KEY `idx_sys_user_dept_status` (`dept_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='用户表';

CREATE TABLE IF NOT EXISTS `sys_user_role` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
  `role_id` BIGINT UNSIGNED NOT NULL COMMENT '角色ID',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_user_role` (`user_id`, `role_id`),
  KEY `idx_sys_user_role_role_id` (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='用户角色关联表';

CREATE TABLE IF NOT EXISTS `sys_role_menu` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `role_id` BIGINT UNSIGNED NOT NULL COMMENT '角色ID',
  `menu_id` BIGINT UNSIGNED NOT NULL COMMENT '菜单ID',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_role_menu` (`role_id`, `menu_id`),
  KEY `idx_sys_role_menu_menu_id` (`menu_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='角色菜单关联表';

CREATE TABLE IF NOT EXISTS `sys_user_post` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
  `post_id` BIGINT UNSIGNED NOT NULL COMMENT '岗位ID',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_user_post` (`user_id`, `post_id`),
  KEY `idx_sys_user_post_post_id` (`post_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='用户岗位关联表';

CREATE TABLE IF NOT EXISTS `sys_dict_type` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '字典类型ID',
  `dict_name` VARCHAR(100) NOT NULL COMMENT '字典名称',
  `dict_type` VARCHAR(100) NOT NULL COMMENT '字典类型编码',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_dict_type` (`dict_type`),
  KEY `idx_sys_dict_type_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='字典类型表';

CREATE TABLE IF NOT EXISTS `sys_dict_data` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '字典数据ID',
  `dict_type_id` BIGINT UNSIGNED NOT NULL COMMENT '字典类型ID',
  `label` VARCHAR(100) NOT NULL COMMENT '字典标签',
  `value` VARCHAR(100) NOT NULL COMMENT '字典键值',
  `tag_type` VARCHAR(30) NULL COMMENT '标签类型，如success/info/warning/danger',
  `tag_class` VARCHAR(100) NULL COMMENT '标签样式类',
  `is_default` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否默认',
  `sort` INT NOT NULL DEFAULT 0 COMMENT '排序',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_dict_data` (`dict_type_id`, `value`),
  KEY `idx_sys_dict_data_type_status` (`dict_type_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='字典数据表';

CREATE TABLE IF NOT EXISTS `sys_config` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '参数ID',
  `config_name` VARCHAR(100) NOT NULL COMMENT '参数名称',
  `config_key` VARCHAR(100) NOT NULL COMMENT '参数键名',
  `config_value` TEXT NOT NULL COMMENT '参数键值',
  `config_type` TINYINT NOT NULL DEFAULT 0 COMMENT '配置类型：0普通 1系统内置',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0停用 1启用',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_config_key` (`config_key`),
  KEY `idx_sys_config_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='参数配置表';

CREATE TABLE IF NOT EXISTS `sys_notice` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '公告ID',
  `title` VARCHAR(200) NOT NULL COMMENT '公告标题',
  `notice_type` TINYINT NOT NULL DEFAULT 1 COMMENT '类型：1通知 2公告',
  `summary` VARCHAR(500) NULL COMMENT '摘要',
  `content` MEDIUMTEXT NOT NULL COMMENT '公告内容',
  `status` TINYINT NOT NULL DEFAULT 0 COMMENT '状态：0草稿 1已发布 2已下线',
  `published_by` BIGINT UNSIGNED NULL COMMENT '发布人',
  `published_at` DATETIME(3) NULL COMMENT '发布时间',
  `version_no` INT NOT NULL DEFAULT 1 COMMENT '版本号',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  KEY `idx_sys_notice_status_published_at` (`status`, `published_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='通知公告表';

-- =========================
-- 2) 日志与监控
-- =========================

CREATE TABLE IF NOT EXISTS `sys_oper_log` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `module` VARCHAR(64) NOT NULL COMMENT '模块名称',
  `business_type` TINYINT NOT NULL COMMENT '业务类型',
  `method` VARCHAR(100) NULL COMMENT '方法名称',
  `request_method` VARCHAR(10) NULL COMMENT '请求方式',
  `operator_type` TINYINT NOT NULL DEFAULT 1 COMMENT '操作类别',
  `oper_name` VARCHAR(64) NULL COMMENT '操作人员',
  `dept_name` VARCHAR(64) NULL COMMENT '部门名称',
  `url` VARCHAR(255) NULL COMMENT '请求URL',
  `ip` VARCHAR(45) NULL COMMENT '主机地址',
  `location` VARCHAR(128) NULL COMMENT '操作地点',
  `request_params` JSON NULL COMMENT '请求参数',
  `response_data` JSON NULL COMMENT '返回参数',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0失败 1成功',
  `error_msg` TEXT NULL COMMENT '错误消息',
  `user_agent` VARCHAR(255) NULL COMMENT '浏览器信息',
  `os` VARCHAR(64) NULL COMMENT '操作系统',
  `duration_ms` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '耗时毫秒',
  `oper_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '操作时间',
  PRIMARY KEY (`id`),
  KEY `idx_sys_oper_log_oper_name` (`oper_name`),
  KEY `idx_sys_oper_log_module_status` (`module`, `status`),
  KEY `idx_sys_oper_log_oper_at` (`oper_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='操作日志表';

CREATE TABLE IF NOT EXISTS `sys_login_log` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `username` VARCHAR(64) NULL COMMENT '用户名',
  `login_type` TINYINT NOT NULL DEFAULT 1 COMMENT '类型：1登录 2退出 3失败',
  `ip` VARCHAR(45) NULL COMMENT '登录IP',
  `location` VARCHAR(128) NULL COMMENT '登录地点',
  `browser` VARCHAR(64) NULL COMMENT '浏览器',
  `os` VARCHAR(64) NULL COMMENT '操作系统',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0失败 1成功',
  `message` VARCHAR(255) NULL COMMENT '提示消息',
  `login_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '登录时间',
  PRIMARY KEY (`id`),
  KEY `idx_sys_login_log_username` (`username`),
  KEY `idx_sys_login_log_status` (`status`),
  KEY `idx_sys_login_log_login_at` (`login_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='登录日志表';

CREATE TABLE IF NOT EXISTS `sys_online_session` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '会话ID',
  `session_token_hash` CHAR(64) NOT NULL COMMENT '令牌哈希',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
  `username` VARCHAR(64) NOT NULL COMMENT '用户名',
  `ip` VARCHAR(45) NULL COMMENT '登录IP',
  `location` VARCHAR(128) NULL COMMENT '登录地点',
  `browser` VARCHAR(64) NULL COMMENT '浏览器',
  `os` VARCHAR(64) NULL COMMENT '操作系统',
  `login_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '登录时间',
  `last_active_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '最后活跃时间',
  `expires_at` DATETIME(3) NOT NULL COMMENT '过期时间',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0失效 1在线',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_online_session_token` (`session_token_hash`),
  KEY `idx_sys_online_session_user` (`user_id`),
  KEY `idx_sys_online_session_status_active` (`status`, `last_active_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='在线会话表';

CREATE TABLE IF NOT EXISTS `sys_job` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '任务ID',
  `job_name` VARCHAR(128) NOT NULL COMMENT '任务名称',
  `job_group` VARCHAR(64) NOT NULL DEFAULT 'DEFAULT' COMMENT '任务组名',
  `invoke_target` VARCHAR(255) NOT NULL COMMENT '调用目标',
  `cron_expression` VARCHAR(64) NOT NULL COMMENT 'Cron表达式',
  `misfire_policy` TINYINT NOT NULL DEFAULT 1 COMMENT '错失策略',
  `concurrent` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否并发执行',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0暂停 1运行',
  `remark` VARCHAR(500) NULL COMMENT '备注',
  `last_run_at` DATETIME(3) NULL COMMENT '最后执行时间',
  `next_run_at` DATETIME(3) NULL COMMENT '下次执行时间',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_sys_job_name_group` (`job_name`, `job_group`),
  KEY `idx_sys_job_status_next` (`status`, `next_run_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='定时任务表';

CREATE TABLE IF NOT EXISTS `sys_job_log` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `job_id` BIGINT UNSIGNED NOT NULL COMMENT '任务ID',
  `job_name` VARCHAR(128) NOT NULL COMMENT '任务名称',
  `job_group` VARCHAR(64) NOT NULL COMMENT '任务组名',
  `invoke_target` VARCHAR(255) NOT NULL COMMENT '调用目标',
  `cron_expression` VARCHAR(64) NOT NULL COMMENT 'Cron表达式',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0失败 1成功',
  `message` VARCHAR(500) NULL COMMENT '日志消息',
  `exception_info` TEXT NULL COMMENT '异常信息',
  `trigger_type` VARCHAR(16) NOT NULL DEFAULT 'auto' COMMENT '触发方式：auto自动 manual手动',
  `started_at` DATETIME(3) NOT NULL COMMENT '开始时间',
  `finished_at` DATETIME(3) NULL COMMENT '结束时间',
  `duration_ms` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '执行耗时',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_sys_job_log_job_id` (`job_id`),
  KEY `idx_sys_job_log_status` (`status`),
  KEY `idx_sys_job_log_started_at` (`started_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='定时任务日志表';

-- =========================
-- 3) AI 页面（首期Mock）
-- =========================

CREATE TABLE IF NOT EXISTS `ai_session` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '会话ID',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '创建人用户ID',
  `title` VARCHAR(200) NOT NULL COMMENT '会话标题',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：0关闭 1进行中',
  `last_active_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '最后活跃时间',
  `created_by` BIGINT UNSIGNED NULL COMMENT '创建人',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  `updated_by` BIGINT UNSIGNED NULL COMMENT '更新人',
  `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT '更新时间',
  `is_deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  KEY `idx_ai_session_user_active` (`user_id`, `last_active_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='AI会话表';

CREATE TABLE IF NOT EXISTS `ai_message` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '消息ID',
  `session_id` BIGINT UNSIGNED NOT NULL COMMENT '会话ID',
  `role` TINYINT NOT NULL COMMENT '角色：1用户 2助手 3系统',
  `content` MEDIUMTEXT NOT NULL COMMENT '消息内容',
  `content_format` TINYINT NOT NULL DEFAULT 1 COMMENT '格式：1Markdown 2PlainText',
  `is_mock` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否Mock消息',
  `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_ai_message_session_time` (`session_id`, `created_at`),
  KEY `idx_ai_message_role` (`role`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='AI消息表';

-- =========================
-- 4) 初始化建议
-- =========================
-- 1. 初始化超级管理员角色与账号。
-- 2. 初始化系统管理/日志管理/系统监控/AI对话菜单。
-- 3. 初始化基础参数与字典项。
