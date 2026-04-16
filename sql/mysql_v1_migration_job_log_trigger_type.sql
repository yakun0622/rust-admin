-- 为 sys_job_log 增加 trigger_type 字段（幂等）
SET @db_name = DATABASE();
SET @column_exists = (
  SELECT COUNT(1)
  FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = @db_name
    AND TABLE_NAME = 'sys_job_log'
    AND COLUMN_NAME = 'trigger_type'
);

SET @ddl = IF(
  @column_exists = 0,
  "ALTER TABLE sys_job_log ADD COLUMN trigger_type VARCHAR(16) NOT NULL DEFAULT 'auto' COMMENT '触发方式：auto自动 manual手动' AFTER exception_info",
  "SELECT 'sys_job_log.trigger_type already exists' AS message"
);

PREPARE stmt FROM @ddl;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 可选：迁移历史日志前缀到 trigger_type 字段
UPDATE sys_job_log
SET trigger_type = 'manual'
WHERE trigger_type = 'auto' AND message LIKE '[manual]%';

UPDATE sys_job_log
SET trigger_type = 'auto'
WHERE trigger_type = 'auto' AND message LIKE '[auto]%';

UPDATE sys_job_log
SET message = TRIM(SUBSTRING(message, LOCATE('] ', message) + 1))
WHERE message LIKE '[manual] %' OR message LIKE '[auto] %';
