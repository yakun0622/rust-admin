# Security Policy

## Supported Versions

我们优先维护以下版本：

- `main` 分支最新代码
- 最新发布的正式版本（Latest Release）

## Reporting a Vulnerability

请不要在公开 Issue 直接披露安全漏洞细节。

建议方式：

1. 使用 GitHub 的 Private Vulnerability Reporting（私密漏洞报告）。
2. 在报告中提供：
   - 漏洞影响范围
   - 复现步骤
   - 可能的修复建议（可选）

## Response SLA

- 维护者会在 72 小时内确认是否收到报告。
- 会根据风险等级安排修复并在必要时发布安全公告。

## Scope

当前重点关注：

- 鉴权与权限绕过
- JWT 与会话安全
- SQL 注入与命令注入
- 敏感信息泄露（配置、日志、调试输出）
