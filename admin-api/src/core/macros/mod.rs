#[macro_export]
macro_rules! api_request {
    ($request:expr, $params:expr) => {{
        let __request = &$request;
        tracing::info!(
            target: "api_request",
            method = %__request.method(),
            url = %__request.uri(),
            params = %$params,
            "incoming api request"
        );
    }};
}
