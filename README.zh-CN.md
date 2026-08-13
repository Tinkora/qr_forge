# qr_forge

[English](./README.md)

[![CI](https://github.com/Tinkora/qr_forge/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/qr_forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.zh-CN.md)

[![在 Ko-fi 上支持 Tinkora](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

qr_forge 是一个在浏览器本地生成二维码、Code 128 条形码和 EAN-13 条形码的工具。它使用 Rust 和 WebAssembly 输出 SVG 与 PNG 文件，不会把待编码内容发送到应用服务器。

[打开在线应用](https://tinkora.github.io/qr_forge/)

## 为什么使用 qr_forge

二维码内容经常包含 Wi-Fi 凭据、联系人信息或内部网址。qr_forge 在浏览器内完成生成并以静态应用形式发布，因此应用不会上传这些内容，也不依赖第三方生成 API。

## 功能

- 支持 L、M、Q、H 四种纠错级别的二维码
- 支持网址或文本、Wi-Fi、vCard 3.0、电话和邮件输入模式
- 支持可打印 ASCII 的 Code 128；偶数长度纯数字输入使用更紧凑的子集 C
- 从恰好 12 位数字生成 EAN-13，并自动计算校验位
- 输出 SVG 和 PNG，可配置颜色、模块尺寸和静区
- 支持 PNG Logo 叠加，尺寸为二维码数据区的 5% 至 30%
- 默认英文界面，可在应用内切换为简体中文
- 纯静态、浏览器本地处理，不需要应用后端

有关明确的非目标，请阅读[产品范围](./docs/PRODUCT_SCOPE.zh-CN.md)；有关限制、输出和错误码，请阅读[产品契约](./docs/CONTRACT.zh-CN.md)。

## 快速开始

环境要求：

- Rust 1.85 或更高版本
- Rust 的 `wasm32-unknown-unknown` target
- `wasm-pack` 0.15 或更高版本
- Python 3 或其他本地静态 HTTP 服务器

```bash
git clone https://github.com/Tinkora/qr_forge.git
cd qr_forge
rustup target add wasm32-unknown-unknown

wasm-pack build --target web crates/qr_forge_web -- --locked
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

打开 `http://localhost:8080`。应用需要通过本地 HTTP 服务器加载 JavaScript module 和 WebAssembly 资源，不能直接双击 HTML 文件运行。

## 开发

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

浏览器测试需要 Node.js 24 或更高版本。即使本地安装了更新的编译器，仓库仍须保持与 Rust 1.85 兼容。修改任何 HTML 或面向用户的前端之前，必须遵守 [AGENTS.md](./AGENTS.md) 中的 `ui-ux-pro-max` 和真实浏览器验证规则。

## 项目结构

| 路径 | 职责 |
| ------ | ------ |
| `crates/qr_forge_core` | 二维码和条形码的生成、校验、渲染及 WASM 函数 |
| `crates/qr_forge_web` | WebAssembly package 入口和静态浏览器应用 |
| `docs` | 产品范围、公开契约、成熟度和发布流程 |
| `.github` | 贡献模板及自动化质量、安全和发布 workflow |

## 文档

- [产品范围](./docs/PRODUCT_SCOPE.zh-CN.md)
- [产品契约](./docs/CONTRACT.zh-CN.md)
- [成熟度与兼容性](./docs/MATURITY.zh-CN.md)
- [发布检查清单](./docs/RELEASE_CHECKLIST.zh-CN.md)
- [贡献指南](./CONTRIBUTING.zh-CN.md)
- [安全策略](./SECURITY.zh-CN.md)
- [支持渠道](./SUPPORT.zh-CN.md)
- [维护者](./MAINTAINERS.md)
- [变更日志](./CHANGELOG.md)

## 隐私与安全

应用不会主动传输二维码或条形码输入。托管服务商在提供静态文件时仍可能收到常规请求元数据。安全模型和私密报告渠道请参阅 [SECURITY.zh-CN.md](./SECURITY.zh-CN.md)。

## 许可证

项目采用 [MIT License](./LICENSE)。Copyright (c) Tinkora contributors.
