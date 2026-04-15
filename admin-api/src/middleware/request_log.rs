use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{
        header::{CONTENT_LENGTH, CONTENT_TYPE},
        Method,
    },
    middleware::Next,
    response::Response,
};
use serde_json::{Map, Value};

const MAX_LOG_BODY_BYTES: usize = 64 * 1024;
const SENSITIVE_MASK: &str = "***";
const SENSITIVE_KEYS: &[&str] = &[
    "password",
    "passwd",
    "old_password",
    "new_password",
    "confirm_password",
    "password_hash",
];

pub async fn log_api_request(request: Request, next: Next) -> Response {
    let (request, params) = capture_request_params(request).await;
    crate::api_request!(request, params);
    next.run(request).await
}

async fn capture_request_params(request: Request) -> (Request, String) {
    let query = request.uri().query().unwrap_or_default().to_string();

    if should_skip_body(&request) {
        let params = build_params_payload(&query, None, None);
        return (request, params);
    }

    let content_type = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();

    let (parts, body) = request.into_parts();
    let body_bytes = match to_bytes(body, MAX_LOG_BODY_BYTES).await {
        Ok(bytes) => bytes,
        Err(err) => {
            let request = Request::from_parts(parts, Body::empty());
            let params =
                build_params_payload(&query, None, Some(format!("body_read_error: {err}")));
            return (request, params);
        }
    };

    let body_param = parse_body_param(&content_type, &body_bytes);
    let request = Request::from_parts(parts, Body::from(body_bytes));
    let params = build_params_payload(&query, body_param, None);
    (request, params)
}

fn should_skip_body(request: &Request) -> bool {
    let method = request.method();
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
        return true;
    }

    let content_type = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if content_type.starts_with("multipart/")
        || content_type.starts_with("application/octet-stream")
    {
        return true;
    }

    let content_length = request
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    content_length > MAX_LOG_BODY_BYTES
}

fn build_params_payload(query: &str, body: Option<Value>, note: Option<String>) -> String {
    let mut params = Map::new();

    if !query.is_empty() {
        params.insert("query".into(), Value::String(mask_query(query)));
    }
    if let Some(body) = body {
        params.insert("body".into(), body);
    }
    if let Some(note) = note {
        params.insert("note".into(), Value::String(note));
    }

    if params.is_empty() {
        "-".to_string()
    } else {
        Value::Object(params).to_string()
    }
}

fn parse_body_param(content_type: &str, body_bytes: &[u8]) -> Option<Value> {
    if body_bytes.is_empty() {
        return None;
    }

    if content_type.contains("application/json") {
        if let Ok(mut json) = serde_json::from_slice::<Value>(body_bytes) {
            mask_json_sensitive_fields(&mut json);
            return Some(json);
        }
    }

    let mut body_raw = String::from_utf8_lossy(body_bytes).to_string();
    if body_raw.len() > 2000 {
        body_raw.truncate(2000);
        body_raw.push_str("...(truncated)");
    }
    Some(Value::String(body_raw))
}

fn mask_query(query: &str) -> String {
    query
        .split('&')
        .map(|segment| match segment.split_once('=') {
            Some((key, _value)) if is_sensitive_key(key) => format!("{key}={SENSITIVE_MASK}"),
            Some((key, value)) => format!("{key}={value}"),
            None => segment.to_string(),
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn mask_json_sensitive_fields(value: &mut Value) {
    match value {
        Value::Object(object) => {
            for (key, value) in object.iter_mut() {
                if is_sensitive_key(key) {
                    *value = Value::String(SENSITIVE_MASK.to_string());
                } else {
                    mask_json_sensitive_fields(value);
                }
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                mask_json_sensitive_fields(item);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key_lower = key.trim().to_ascii_lowercase();
    SENSITIVE_KEYS
        .iter()
        .any(|item| *item == key_lower.as_str())
}
