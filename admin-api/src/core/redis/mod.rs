use redis::aio::MultiplexedConnection;

use crate::core::errors::AppError;

#[derive(Clone)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    pub async fn get_connection(&self) -> Result<MultiplexedConnection, AppError> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| AppError::internal(format!("获取 Redis 连接失败: {err}")))
    }

    pub async fn ping(&self) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        let pong: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("Redis PING 失败: {err}")))?;
        if pong != "PONG" {
            return Err(AppError::internal(format!(
                "redis ping returned unexpected result: {pong}"
            )));
        }
        Ok(())
    }

    pub async fn set_string(&self, key: &str, value: impl Into<String>) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value.into())
            .query_async::<String>(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("写入 Redis key 失败(key={key}): {err}")))?;
        Ok(())
    }

    pub async fn get_string_opt(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.get_connection().await?;
        redis::cmd("GET")
            .arg(key)
            .query_async::<Option<String>>(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("读取 Redis key 失败(key={key}): {err}")))
    }

    pub async fn key_type(&self, key: &str) -> Result<String, AppError> {
        let mut conn = self.get_connection().await?;
        redis::cmd("TYPE")
            .arg(key)
            .query_async::<String>(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("读取 Redis key 类型失败(key={key}): {err}")))
    }

    pub async fn ttl_secs(&self, key: &str) -> Result<i64, AppError> {
        let mut conn = self.get_connection().await?;
        redis::cmd("TTL")
            .arg(key)
            .query_async::<i64>(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("读取 Redis TTL 失败(key={key}): {err}")))
    }

    pub async fn scan_keys(
        &self,
        pattern: &str,
        limit: usize,
        count: usize,
    ) -> Result<Vec<String>, AppError> {
        let mut conn = self.get_connection().await?;
        let safe_limit = limit.clamp(1, 10_000);
        let safe_count = count.clamp(1, 5_000);

        let mut cursor = 0_u64;
        let mut keys = Vec::new();
        while keys.len() < safe_limit {
            let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(safe_count)
                .query_async(&mut conn)
                .await
                .map_err(|err| AppError::internal(format!("扫描 Redis Key 失败: {err}")))?;

            for key in batch {
                keys.push(key);
                if keys.len() >= safe_limit {
                    break;
                }
            }
            if next_cursor == 0 {
                break;
            }
            cursor = next_cursor;
        }

        Ok(keys)
    }

    pub async fn try_lock(
        &self,
        lock_key: &str,
        lock_token: &str,
        ttl_secs: u64,
    ) -> Result<bool, AppError> {
        let mut conn = self.get_connection().await?;
        let set_result = redis::cmd("SET")
            .arg(lock_key)
            .arg(lock_token)
            .arg("NX")
            .arg("EX")
            .arg(ttl_secs)
            .query_async::<Option<String>>(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("获取任务锁失败: {err}")))?;
        Ok(set_result.is_some())
    }

    pub async fn release_lock_if_owner(
        &self,
        lock_key: &str,
        lock_token: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        let script = redis::Script::new(
            r#"
if redis.call("GET", KEYS[1]) == ARGV[1] then
  return redis.call("DEL", KEYS[1])
else
  return 0
end
"#,
        );
        let _deleted: i32 = script
            .key(lock_key)
            .arg(lock_token)
            .invoke_async(&mut conn)
            .await
            .map_err(|err| AppError::internal(format!("释放任务锁失败: {err}")))?;
        Ok(())
    }
}
