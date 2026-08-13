# qr_forge

[简体中文](./README.zh-CN.md)

[![CI](https://github.com/Tinkora/qr_forge/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/qr_forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

[![Support Tinkora on Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

qr_forge creates QR codes, Code 128 barcodes, and EAN-13 barcodes locally in a browser. It uses Rust and WebAssembly to produce SVG and PNG files without sending the content being encoded to an application server.

[Open the web app](https://tinkora.github.io/qr_forge/)

## Why qr_forge

QR payloads often contain Wi-Fi credentials, contact details, or internal URLs. qr_forge keeps generation in the browser and ships as a static application, so the application does not upload those values or require a third-party generation API.

## Features

- QR codes with L, M, Q, and H error correction
- URL or text, Wi-Fi, vCard 3.0, phone, and email input modes
- Code 128 for printable ASCII, with compact subset C for even-length numeric input
- EAN-13 from exactly 12 digits, with the check digit calculated automatically
- SVG and PNG output with configurable colors, module size, and quiet zone
- Optional PNG logo overlay at 5% to 30% of the QR data area
- English interface by default with an in-app Simplified Chinese switch
- Static, browser-local processing with no application backend

See [Product Scope](./docs/PRODUCT_SCOPE.md) for explicit non-goals and [Product Contract](./docs/CONTRACT.md) for limits, outputs, and error codes.

## Quick Start

Requirements:

- Rust 1.85 or newer
- The `wasm32-unknown-unknown` Rust target
- `wasm-pack` 0.15 or newer
- Python 3 or another local static HTTP server

```bash
git clone https://github.com/Tinkora/qr_forge.git
cd qr_forge
rustup target add wasm32-unknown-unknown

wasm-pack build --target web crates/qr_forge_web -- --locked
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

Open `http://localhost:8080`. A local HTTP server is required because the app loads JavaScript modules and WebAssembly assets.

## Development

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

Browser tests require Node.js 24 or newer. The repository requires Rust 1.85 compatibility even if a newer compiler is installed locally. Before changing any HTML or user-facing frontend, follow the `ui-ux-pro-max` and browser verification rules in [AGENTS.md](./AGENTS.md).

## Project Layout

| Path | Responsibility |
| ------ | ---------------- |
| `crates/qr_forge_core` | QR and barcode generation, validation, rendering, and WASM functions |
| `crates/qr_forge_web` | WebAssembly package entry point and static browser application |
| `docs` | Product scope, public contract, maturity, and release process |
| `.github` | Contribution templates and automated quality, security, and release workflows |

## Documentation

- [Product Scope](./docs/PRODUCT_SCOPE.md)
- [Product Contract](./docs/CONTRACT.md)
- [Maturity and Compatibility](./docs/MATURITY.md)
- [Release Checklist](./docs/RELEASE_CHECKLIST.md)
- [Contributing](./CONTRIBUTING.md)
- [Security](./SECURITY.md)
- [Support](./SUPPORT.md)
- [Maintainers](./MAINTAINERS.md)
- [Changelog](./CHANGELOG.md)

## Privacy and Security

The application does not intentionally transmit QR or barcode input. The hosting provider can still receive ordinary request metadata when serving the static files. Review [SECURITY.md](./SECURITY.md) for the security model and private reporting channel.

## License

Licensed under the [MIT License](./LICENSE). Copyright (c) Tinkora contributors.
