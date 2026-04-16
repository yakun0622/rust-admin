use serde::{Deserialize, Serialize};
use tracing::error;

/// IP 所在地查询接口返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpLocationInfo {
    pub ip: String,
    pub ip_type: Option<String>,
    pub country: String,
    pub country_code: String,
    pub city: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub isp: String,
    pub organization: String,
    #[serde(default)]
    pub asn: u32,
    pub asn_name: Option<String>,
    pub risk_score: Option<f32>,
    pub risk_level: Option<String>,
    #[serde(default)]
    pub is_vpn: bool,
    #[serde(default)]
    pub is_proxy: bool,
    #[serde(default)]
    pub is_tor: bool,
    #[serde(default)]
    pub is_datacenter: bool,
    #[serde(default)]
    pub is_residential: bool,
    pub timestamp: String,
}

/// 根据 IP 获取物理所在地信息
/// 使用 iprobe.io 提供的接口
pub async fn get_ip_location(ip: &str) -> Option<IpLocationInfo> {
    if ip.is_empty() || ip == "127.0.0.1" || ip == "::1" || ip == "localhost" {
        return None;
    }

    let url = format!("https://iprobe.io/api/check?ip={}", ip);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<IpLocationInfo>().await {
                    Ok(info) => Some(info),
                    Err(e) => {
                        error!("[IP查询解析失败] {}: {:?}", ip, e);
                        None
                    }
                }
            } else {
                error!("[IP查询接口返回错误] {}: {}", ip, response.status());
                None
            }
        }
        Err(e) => {
            error!("[IP查询请求失败] {}: {:?}", ip, e);
            None
        }
    }
}

/// 格式化 IP 所在地为字符串
/// 例: "United States, California, San Jose"
pub fn format_location(info: &IpLocationInfo) -> String {
    let mut parts = Vec::new();

    parts.push(info.country.as_str());

    if let Some(ref r) = info.region {
        if !r.is_empty() {
            parts.push(r.as_str());
        }
    }

    if let Some(ref c) = info.city {
        if !c.is_empty() {
            parts.push(c.as_str());
        }
    }

    if parts.is_empty() {
        "未知地点".to_string()
    } else {
        parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_location() {
        let info = IpLocationInfo {
            ip: "23.247.137.109".to_string(),
            ip_type: Some("datacenter".to_string()),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            city: Some("San Jose".to_string()),
            region: Some("California".to_string()),
            region_code: Some("CA".to_string()),
            district: None,
            postal_code: Some("95192".to_string()),
            latitude: 37.3402,
            longitude: -121.8704,
            timezone: "America/Los_Angeles".to_string(),
            isp: "Black Mesa Corporation".to_string(),
            organization: "Black Mesa Corporation".to_string(),
            asn: 46997,
            asn_name: Some("NATOLAB".to_string()),
            risk_score: Some(60.0),
            risk_level: Some("medium".to_string()),
            is_vpn: false,
            is_proxy: false,
            is_tor: false,
            is_datacenter: true,
            is_residential: false,
            timestamp: "2026-04-16T10:22:46.289Z".to_string(),
        };

        assert_eq!(
            format_location(&info),
            "United States, California, San Jose"
        );
    }
}
