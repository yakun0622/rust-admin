use std::collections::BTreeMap;

use crate::core::{
    errors::AppError, redis::RedisClient,
    vo::monitor_vo::{CacheKeyItemVo, CacheNamespaceItemVo, CacheNamespaceListVo, CacheSearchVo},
};

#[derive(Clone)]
pub struct MonitorCacheService {
    redis_client: RedisClient,
}

impl MonitorCacheService {
    pub fn new(redis_client: RedisClient) -> Self {
        Self { redis_client }
    }

    pub async fn search_cache(
        &self,
        keyword: Option<&str>,
        limit: usize,
    ) -> Result<CacheSearchVo, AppError> {
        let safe_limit = limit.clamp(1, 200);
        let pattern = build_redis_pattern(keyword);
        let keys = self.redis_client.scan_keys(&pattern, safe_limit, 100).await?;
        let mut items = Vec::with_capacity(keys.len());

        for key in keys {
            let data_type = self
                .redis_client
                .key_type(&key)
                .await
                .unwrap_or_else(|_| "unknown".to_string());
            let ttl_secs = self
                .redis_client
                .ttl_secs(&key)
                .await
                .unwrap_or(-2);
            let sample = if data_type == "string" {
                self.redis_client
                    .get_string_opt(&key)
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
        let keys = self.redis_client.scan_keys("*", 500, 100).await?;

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

fn truncate_sample(value: &str) -> String {
    const MAX_LEN: usize = 120;
    if value.len() <= MAX_LEN {
        value.to_string()
    } else {
        format!("{}...", &value[..MAX_LEN])
    }
}
