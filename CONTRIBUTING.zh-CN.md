# 为 qr_forge 做贡献

[English](./CONTRIBUTING.md)

感谢你改进 qr_forge。贡献内容应符合已记录的产品范围，保持浏览器本地处理，并提供能够证明变更行为有效的验证证据。

## 开始之前

- 阅读[产品范围](./docs/PRODUCT_SCOPE.zh-CN.md)、[产品契约](./docs/CONTRACT.zh-CN.md)和[成熟度](./docs/MATURITY.zh-CN.md)。
- 搜索现有 [issues](https://github.com/Tinkora/qr_forge/issues) 和 [discussions](https://github.com/Tinkora/qr_forge/discussions)。
- 大型功能、公开 API 变更、新依赖或产品范围变更应先创建 issue。
- 安全漏洞只能通过 [GitHub 私密漏洞报告](https://github.com/Tinkora/qr_forge/security/advisories/new)提交。

## 开发环境

- Rust 1.85 或更高版本；所有变更必须保持与 Rust 1.85 兼容
- `wasm32-unknown-unknown` target
- `wasm-pack` 0.15 或更高版本
- 用于浏览器测试的 Node.js 24 或更高版本及 npm
- 用于浏览器检查的 Python 3 或其他静态 HTTP 服务器

```bash
rustup target add wasm32-unknown-unknown
```

## 仓库结构

```text
qr_forge/
|-- crates/
|   |-- qr_forge_core/     # 生成、校验、渲染和 WASM 函数
|   `-- qr_forge_web/      # WebAssembly 入口和静态浏览器应用
|-- docs/                  # 范围、契约、成熟度和发布流程
|-- .github/               # 模板和自动化
`-- AGENTS.md              # 维护者和智能体使用的仓库规则
```

## 本地检查

请求评审前运行完整基线：

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p qr_forge_core --target wasm32-unknown-unknown
cargo check -p qr_forge_web --target wasm32-unknown-unknown
wasm-pack build --target web crates/qr_forge_web -- --locked

cd crates/qr_forge_web
npm ci
npx playwright install chromium
npm run test:wasm-smoke:local
```

构建并启动浏览器应用：

```bash
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

修改二维码或条形码行为时，应针对有效输入、无效输入、边界限制，以及适用时由独立解码器验证的输出，增加面向结果的测试。

## 前端变更

创建、修改、评审或调试 HTML 及面向用户的前端代码之前，必须按照 [AGENTS.md](./AGENTS.md) 使用 `ui-ux-pro-max` skill。提交 375、768、1024 和 1440 像素宽度下的真实浏览器证据，并检查键盘操作、焦点可见性、无障碍名称、溢出、重叠、浏览器控制台、网络请求和下载。

未经产品与隐私决策批准，不得增加 CDN 或运行时第三方请求。应用不得传输用户输入的 payload。

## 文档与语言

- 公开文档默认使用英文；存在简体中文版本时，英文文档应提供入口。
- 含义发生变化时，必须在同一个 PR 中同步更新两种语言。
- 代码注释和公开 commit message 只能使用英文。
- 不得把规划中的集成描述为已实现行为。
- Markdown 文件不得使用 emoji。

## Commit

使用英文 [Conventional Commits](https://www.conventionalcommits.org/)，每个 commit 应包含一个逻辑完整的变更。例如：

```text
fix: preserve whitespace in Wi-Fi credentials
docs: clarify the pre-1.0 compatibility policy
```

## Pull Request 流程

1. Fork 仓库，并创建类似 `fix/ean13-validation` 的聚焦分支。
2. 用最小且完整的变更解决 issue。
3. 按需增加或更新测试和中英文文档。
4. 运行所有相关本地检查，并记录无法验证的环境限制。
5. 完整填写 PR 模板，关联 issue，并描述用户可见影响。
6. 使用后续 commit 处理评审意见；维护者可能在 merge 时 squash。

只有在必要检查通过、requested changes 已解决、公开契约准确且不包含无关变更时，PR 才能 merge。

## 评审优先级

评审者按以下顺序评估：

1. 输出正确并可由独立扫描器识别
2. 隐私、输入安全和有界资源使用
3. 与公开契约及 Rust 1.85 的兼容性
4. 浏览器无障碍和可用性
5. 测试质量和长期可维护性

## 社区规范

参与项目须遵守[行为准则](./CODE_OF_CONDUCT.md)。支持和功能讨论应使用 [SUPPORT.zh-CN.md](./SUPPORT.zh-CN.md) 中列出的渠道。
