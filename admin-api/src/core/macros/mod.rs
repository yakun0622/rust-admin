/// 批量装配 `Service(Repo(pool))` 模式（无服务间依赖）。
///
/// ```ignore
/// wire!(pool;
///     sys_role_service: SysRoleService[SysRoleRepository],
///     sys_user_service: SysUserService[SysUserRepository],
/// );
/// ```
#[macro_export]
macro_rules! wire {
    ($pool:expr; $($field:ident : $Svc:ident [$Repo:ident]),+ $(,)?) => {
        $(
            let $field = $Svc::new($Repo::new($pool.clone()));
        )+
    };
}

/// 装配单个带服务依赖的 Service：`Service(Repo(pool), dep1.clone(), dep2.clone(), ...)`。
/// 被依赖的 Service 必须已经通过 `wire!` 或 `wire_dep!` 创建。
///
/// ```ignore
/// wire!(pool;
///     sys_role_service: SysRoleService[SysRoleRepository],
/// );
/// // SysUserService::new(repo, role_service) —— 额外依赖 sys_role_service
/// wire_dep!(pool; sys_user_service: SysUserService[SysUserRepository] + sys_role_service);
/// ```
#[macro_export]
macro_rules! wire_dep {
    ($pool:expr; $field:ident : $Svc:ident [$Repo:ident] + $($dep:ident),+ $(,)?) => {
        let $field = $Svc::new($Repo::new($pool.clone()), $( $dep.clone() ),+);
    };
}

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
