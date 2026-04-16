# Regression Scripts

## smoke_api.sh

用途：对运行中的 `admin-api` 执行一轮接口 smoke 回归。  
覆盖：`health/auth/system(8类资源 CRUD)/log/monitor/ai` 的核心链路。

运行方式：

```bash
bash admin-api/scripts/regression/smoke_api.sh
```

可选环境变量：

```bash
BASE_URL=http://127.0.0.1:8080 \
USERNAME=admin \
PASSWORD=admin123456 \
bash admin-api/scripts/regression/smoke_api.sh
```

说明：

1. 脚本默认要求目标服务已启动并可访问。
2. 若你希望直接执行文件，请先赋权：
```bash
chmod +x admin-api/scripts/regression/smoke_api.sh
```

## permission_api.sh

用途：权限场景回归（有权 / 无权 / 超级权限）。  
覆盖：

1. 超级管理员登录后可看到 `*:*:*` 通配权限。
2. 超级管理员可正常访问 `system:user:view` 受控接口。
3. 新建无角色用户后，读写受控接口返回 `403`。

运行方式：

```bash
bash admin-api/scripts/regression/permission_api.sh
```

可选环境变量：

```bash
BASE_URL=http://127.0.0.1:8080 \
ADMIN_USERNAME=admin \
ADMIN_PASSWORD=admin123456 \
bash admin-api/scripts/regression/permission_api.sh
```

说明：

1. 脚本会创建一个临时无权限用户并在结束时删除。
2. 临时用户默认使用系统内置初始密码（与 `admin` 种子一致）。
