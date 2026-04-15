use std::collections::BTreeMap;

use redis::aio::MultiplexedConnection;

use crate::core::{
    errors::AppError,
    vo::monitor_vo::{CacheKeyItemVo, CacheNamespaceItemVo, CacheNamespaceListVo, CacheSearchVo},
};

use super::MonitorService;

impl MonitorService {
    pub async fn search_cache(
        &self,
        keyword: Option<&str>,
        limit: usize,
    ) -> Result<CacheSearchVo, AppError> {
        let safe_limit = limit.clamp(1, 200);
        let pattern = build_redis_pattern(keyword);
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| AppError::internal(format!("获取 Redis 连接失败: {err}")))?;

        let keys = scan_keys(&mut conn, &pattern, safe_limit)
            .await
            .map_err(|err| AppError::internal(format!("扫描 Redis Key 失败: {err}")))?;
        let mut items = Vec::with_capacity(keys.len());

        for key in keys {
            let data_type = redis::cmd("TYPE")
                .arg(&key)
                .query_async::<String>(&mut conn)
                .await
                .unwrap_or_else(|_| "unknown".to_string());
            let ttl_secs = redis::cmd("TTL")
                .arg(&key)
                .query_async::<i64>(&mut conn)
                .await
                .unwrap_or(-2);
            let sample = if data_type == "string" {
                redis::cmd("GET")
                    .arg(&key)
                    .query_async::<Option<String>>(&mut conn)
                    .await
                    .ok()
                    .flatten()
                    .map(|value| truncate_sample(&value))
                    .unwrap_or_else(|| "-".to_string())
            } else {
                format!("<{data_type}>")
            };

            items.push(CacheKeyItemVo {
                key,
                data_type,
                ttl_secs,
                sample,
            });
        }

        Ok(CacheSearchVo {
            pattern,
            total: items.len(),
            items,
        })
    }

    pub async fn cache_namespace_list(&self) -> Result<CacheNamespaceListVo, AppError> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| AppError::internal(format!("获取 Redis 连接失败: {err}")))?;

        let keys = scan_keys(&mut conn, "*", 500)
            .await
            .map_err(|err| AppError::internal(format!("扫描 Redis Key 失败: {err}")))?;

        let mut namespace_map: BTreeMap<String, (u64, String)> = BTreeMap::new();
        for key in keys {
            let namespace = key
                .split(':')
                .next()
                .filter(|segment| !segment.trim().is_empty())
                .unwrap_or("(root)")
                .to_string();
            let entry = namespace_map
                .entry(namespace)
                .or_insert((0_u64, key.clone()));
            entry.0 += 1;
            if entry.1.is_empty() {
                entry.1 = key;
            }
        }

        let items = namespace_map
            .into_iter()
            .map(
                |(namespace, (key_count, example_key))| CacheNamespaceItemVo {
                    namespace,
                    key_count,
                    example_key,
                },
            )
            .collect::<Vec<_>>();

        Ok(CacheNamespaceListVo {
            total: items.len(),
            items,
        })
    }
}

fn build_redis_pattern(keyword: Option<&str>) -> String {
    let kw = keyword.map(str::trim).unwrap_or_default();
    if kw.is_empty() {
        "*".to_string()
    } else {
        format!("*{kw}*")
    }
}

async fn scan_keys(
    conn: &mut MultiplexedConnection,
    pattern: &str,
    limit: usize,
) -> redis::RedisResult<Vec<String>> {
    let mut cursor = 0_u64;
    let mut keys = Vec::new();
    while keys.len() < limit {
        let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(100)
            .query_async(conn)
            .await?;

        for key in batch {
            keys.push(key);
            if keys.len() >= limit {
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

fn truncate_sample(value: &str) -> String {
    const MAX_LEN: usize = 120;
    if value.len() <= MAX_LEN {
        value.to_string()
    } else {
        format!("{}...", &value[..MAX_LEN])
    }
}
