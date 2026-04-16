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

#[macro_export]
macro_rules! permission {
    ($state:expr, $current_user:expr, $permission:expr $(,)?) => {{
        $crate::middleware::auth::ensure_permission(&$state, &$current_user, $permission).await?;
    }};
}

#[macro_export]
macro_rules! admin_log {
    (
        $state:expr,
        $current_user:expr,
        $name:expr,
        $business_type:expr,
        $action:expr
    ) => {{
        let __state = $state.clone();
        let __current_user = $current_user.clone();
        let __name = $name.to_string();
        let __business_type: i8 = $business_type as i8;
        let __request_params: Option<String> = None;
        let __start = std::time::Instant::now();
        let __result: Result<_, $crate::core::errors::AppError> = $action.await;
        let __duration_ms = __start.elapsed().as_millis().min(u32::MAX as u128) as u32;
        let __request_method = match __business_type {
            1 => Some("POST".to_string()),
            2 => Some("PUT".to_string()),
            3 => Some("DELETE".to_string()),
            4 => Some("POST".to_string()),
            _ => Some("POST".to_string()),
        };
        let (__status, __error_msg, __response_data) = match &__result {
            Ok(_) => (
                1_i8,
                None,
                Some(serde_json::json!({ "code": 200, "message": "ok" }).to_string()),
            ),
            Err(err) => (
                0_i8,
                Some(err.message.clone()),
                Some(serde_json::json!({ "code": err.code, "message": err.message }).to_string()),
            ),
        };

        let __log_input = $crate::core::model::log::OperLogCreatePo {
            module: module_path!().to_string(),
            business_type: __business_type,
            method: Some(__name),
            request_method: __request_method,
            operator_type: 1,
            oper_name: Some(__current_user.username().to_string()),
            dept_name: None,
            url: None,
            ip: None,
            location: None,
            request_params: __request_params,
            response_data: __response_data,
            status: __status,
            error_msg: __error_msg,
            user_agent: None,
            os: None,
            duration_ms: __duration_ms,
        };

        let _ = __state.log_service().append_oper_log(__log_input).await;
        __result
    }};
    (
        $state:expr,
        $current_user:expr,
        $name:expr,
        $business_type:expr,
        $request_params:expr,
        $action:expr
    ) => {{
        let __state = $state.clone();
        let __current_user = $current_user.clone();
        let __name = $name.to_string();
        let __business_type: i8 = $business_type as i8;
        let __request_params: Option<String> = $request_params;
        let __start = std::time::Instant::now();
        let __result: Result<_, $crate::core::errors::AppError> = $action.await;
        let __duration_ms = __start.elapsed().as_millis().min(u32::MAX as u128) as u32;
        let __request_method = match __business_type {
            1 => Some("POST".to_string()),
            2 => Some("PUT".to_string()),
            3 => Some("DELETE".to_string()),
            4 => Some("POST".to_string()),
            _ => Some("POST".to_string()),
        };
        let (__status, __error_msg, __response_data) = match &__result {
            Ok(_) => (
                1_i8,
                None,
                Some(serde_json::json!({ "code": 200, "message": "ok" }).to_string()),
            ),
            Err(err) => (
                0_i8,
                Some(err.message.clone()),
                Some(serde_json::json!({ "code": err.code, "message": err.message }).to_string()),
            ),
        };

        let __log_input = $crate::core::model::log::OperLogCreatePo {
            module: module_path!().to_string(),
            business_type: __business_type,
            method: Some(__name),
            request_method: __request_method,
            operator_type: 1,
            oper_name: Some(__current_user.username().to_string()),
            dept_name: None,
            url: None,
            ip: None,
            location: None,
            request_params: __request_params,
            response_data: __response_data,
            status: __status,
            error_msg: __error_msg,
            user_agent: None,
            os: None,
            duration_ms: __duration_ms,
        };

        let _ = __state.log_service().append_oper_log(__log_input).await;
        __result
    }};
}

#[macro_export]
macro_rules! admin_oper_log {
    (
        $state:expr,
        $current_user:expr,
        $module:expr,
        $business_type:expr,
        $request_method:expr,
        $url:expr,
        $request_params:expr,
        $action:expr
    ) => {{
        let __state = $state.clone();
        let __current_user = $current_user.clone();
        let __module = $module.to_string();
        let __request_method = $request_method.to_string();
        let __url = $url.to_string();
        let __request_params: Option<String> = $request_params;
        let __start = std::time::Instant::now();
        let __result = $action.await;
        let __duration_ms = __start.elapsed().as_millis().min(u32::MAX as u128) as u32;
        let (__status, __error_msg, __response_data) = match &__result {
            Ok(_) => (
                1_i8,
                None,
                Some(serde_json::json!({ "code": 200, "message": "ok" }).to_string()),
            ),
            Err(err) => (
                0_i8,
                Some(err.message.clone()),
                Some(serde_json::json!({ "code": err.code, "message": err.message }).to_string()),
            ),
        };

        let __log_input = $crate::core::model::log::OperLogCreatePo {
            module: __module,
            business_type: $business_type as i8,
            method: None,
            request_method: Some(__request_method),
            operator_type: 1,
            oper_name: Some(__current_user.username().to_string()),
            dept_name: None,
            url: Some(__url),
            ip: None,
            location: None,
            request_params: __request_params,
            response_data: __response_data,
            status: __status,
            error_msg: __error_msg,
            user_agent: None,
            os: None,
            duration_ms: __duration_ms,
        };

        let _ = __state.log_service().append_oper_log(__log_input).await;
        __result
    }};
}
