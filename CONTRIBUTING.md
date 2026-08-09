# Contributing to qr_forge

[简体中文](./CONTRIBUTING.zh-CN.md)

Thank you for improving qr_forge. Contributions should stay within the documented product scope, preserve browser-local processing, and include evidence that the changed behavior works.

## Before Starting

- Read [Product Scope](./docs/PRODUCT_SCOPE.md), [Product Contract](./docs/CONTRACT.md), and [Maturity](./docs/MATURITY.md).
- Search existing [issues](https://github.com/Tinkora/qr_forge/issues) and [discussions](https://github.com/Tinkora/qr_forge/discussions).
- Open an issue before a large feature, public API change, new dependency, or product-scope change.
- Report vulnerabilities only through [GitHub private vulnerability reporting](https://github.com/Tinkora/qr_forge/security/advisories/new).

## Development Environment

- Rust 1.85 or newer; changes must remain compatible with Rust 1.85
- `wasm32-unknown-unknown` target
- `wasm-pack` 0.15 or newer
- Node.js 24 or newer and npm for browser tests
- Python 3 or another static HTTP server for browser checks

```bash
rustup target add wasm32-unknown-unknown
```

## Repository Layout

```text
qr_forge/
|-- crates/
|   |-- qr_forge_core/     # Generation, validation, rendering, and WASM functions
|   `-- qr_forge_web/      # WebAssembly entry point and static browser app
|-- docs/                  # Scope, contract, maturity, and release process
|-- .github/               # Templates and automation
`-- AGENTS.md              # Repository rules for maintainers and agents
```

## Local Checks

Run the complete baseline before requesting review:

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

Build and serve the browser application:

```bash
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

For QR or barcode behavior changes, add outcome-focused tests for valid input, invalid input, limits, and independently decoded output where applicable.

## Frontend Changes

Before creating, modifying, reviewing, or debugging HTML or user-facing frontend code, use the `ui-ux-pro-max` skill as required by [AGENTS.md](./AGENTS.md). Include real-browser evidence at 375, 768, 1024, and 1440 pixel widths. Check keyboard operation, focus visibility, accessible names, overflow, overlap, browser console output, network requests, and downloads.

Do not add a CDN or runtime third-party request without an approved product and privacy decision. The application must not transmit user-entered payloads.

## Documentation and Language

- Public documentation defaults to English and links to a complete Simplified Chinese counterpart where one exists.
- Update both language versions in the same pull request when meaning changes.
- Code comments and public commit messages are English only.
- Do not document planned integrations as implemented behavior.
- Do not add emoji to Markdown files.

## Commits

Use English [Conventional Commits](https://www.conventionalcommits.org/) and keep each commit logically complete. Examples:

```text
fix: preserve whitespace in Wi-Fi credentials
docs: clarify the pre-1.0 compatibility policy
```

## Pull Request Process

1. Fork the repository and create a focused branch such as `fix/ean13-validation`.
2. Make the smallest complete change that addresses the issue.
3. Add or update tests and English/Chinese documentation as required.
4. Run all relevant local checks and record any environment limitation.
5. Complete the pull request template, link the issue, and describe user-visible effects.
6. Respond to review feedback with additional commits. Maintainers may squash during merge.

A pull request is ready to merge only when required checks pass, requested changes are resolved, the public contract is accurate, and no unrelated changes are included.

## Review Priorities

Reviewers evaluate, in order:

1. Correct and independently scannable output
2. Privacy, input safety, and bounded resource use
3. Compatibility with the public contract and Rust 1.85
4. Browser accessibility and usability
5. Test quality and long-term maintainability

## Community Standards

Participation is governed by the [Code of Conduct](./CODE_OF_CONDUCT.md). Support and feature discussions belong in the channels described in [SUPPORT.md](./SUPPORT.md).
