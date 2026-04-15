use crate::core::dbal::dialect::SqlDialect;

pub fn keyword_like(keyword: Option<&str>) -> Option<String> {
    let kw = keyword.unwrap_or_default().trim();
    if kw.is_empty() {
        None
    } else {
        Some(format!("%{kw}%"))
    }
}

pub fn keyword_args(keyword: Option<&str>) -> (String, String) {
    let kw = keyword.unwrap_or_default().trim().to_string();
    let like = if kw.is_empty() {
        "%".to_string()
    } else {
        format!("%{kw}%")
    };
    (kw, like)
}

pub fn like_condition(column: &str, dialect: &dyn SqlDialect) -> String {
    format!("{column} {} ?", dialect.like_operator())
}

pub fn soft_delete_filter(alias: Option<&str>) -> String {
    match alias {
        Some(prefix) if !prefix.trim().is_empty() => format!("{prefix}.is_deleted = 0"),
        _ => "is_deleted = 0".to_string(),
    }
}
