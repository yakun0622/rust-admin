# Contributing Guide

感谢你愿意参与 RustAdmin 的建设。

## 开发环境

- Rust stable（建议 >= 1.75）
- Node.js >= 18（建议 20）
- MySQL 8.x
- Redis 7.x

## 本地启动

1. Fork 并克隆仓库。
2. 初始化数据库：
   - `sql/rust_admin_20260403.sql`
3. 启动后端：
   - `cd admin-api && APP_ENV=dev cargo run`
4. 启动前端：
   - `cd admin-web && npm install && npm run dev`

## 分支与提交

- 建议从 `main` 拉取功能分支：`feat/xxx`、`fix/xxx`
- 提交信息建议使用 Conventional Commits：
  - `feat: ...`
  - `fix: ...`
  - `refactor: ...`
  - `docs: ...`
  - `chore: ...`

## 代码质量要求

提交 PR 前请确保以下命令通过：

后端：

```bash
cd admin-api
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --all-features
```

前端：

```bash
cd admin-web
npm run typecheck
npm run build
```

## Pull Request 要求

- PR 描述需包含：
  - 变更背景
  - 主要改动点
  - 验证方式
  - 风险与回滚方案（如有）
- UI 相关变更建议附截图或录屏。
- 若涉及数据库变更，请提供迁移脚本与兼容策略。

## 行为与沟通

请遵循 [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md)。
