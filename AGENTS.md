# Repository Guide for AI Agents

## Project Overview

qr_forge is a browser-local QR code and barcode generator built with Rust and WebAssembly. The implemented formats are QR, Code 128, and EAN-13. Outputs are SVG and PNG; QR PNG output can include a local PNG logo.

Do not claim that this repository implements Data Matrix, an MCP server, an Agent Skill, a CLI, a hosted API, batch generation, or package-registry distribution. Proposed work is not shipped behavior.

## Public Content Rules

- Public documentation defaults to English and links to a complete Simplified Chinese counterpart where one exists.
- Update both language versions in the same change when meaning changes.
- Write all new or modified code comments in English.
- Do not include internal migration history, old organization names, local absolute paths, credentials, or private operational notes.
- Do not use emoji in Markdown.
- Treat [docs/CONTRACT.md](./docs/CONTRACT.md) as the normative product contract and keep it synchronized with behavior and tests.

## Architecture

```text
qr_forge/
|-- crates/
|   |-- qr_forge_core/
|   |   |-- src/             # Generation, validation, rendering, and WASM bindings
|   |   `-- tests/           # Boundary and independent scanner tests
|   `-- qr_forge_web/
|       |-- src/lib.rs       # WASM package entry point and core re-exports
|       `-- static/          # English-first bilingual browser application
|-- docs/                    # Scope, contract, maturity, and release checklist
|-- .github/                 # Community and automation configuration
|-- Cargo.toml               # Rust 1.85 workspace and shared dependencies
`-- AGENTS.md                # This repository-specific guide
```

`qr_forge_core` owns product rules. The browser must use the WASM payload builders for Wi-Fi and vCard instead of duplicating escaping logic in JavaScript.

## Key Files

| File | Responsibility |
| ------ | ---------------- |
| `crates/qr_forge_core/src/qr.rs` | QR options, generation, Wi-Fi payloads, and vCard payloads |
| `crates/qr_forge_core/src/barcode.rs` | Code 128 and EAN-13 validation, encoding, and quiet zones |
| `crates/qr_forge_core/src/render.rs` | SVG, PNG, and PNG logo rendering |
| `crates/qr_forge_core/src/error.rs` | `CoreError` and stable machine-readable codes |
| `crates/qr_forge_core/src/wasm.rs` | JavaScript ABI exported through `wasm-bindgen` |
| `crates/qr_forge_core/tests/scanner_compatibility.rs` | Independent decoding tests |
| `crates/qr_forge_core/tests/input_boundaries.rs` | Input, quiet-zone, whitespace, and logo-ratio regression tests |
| `crates/qr_forge_web/static/index.html` | Semantic application structure |
| `crates/qr_forge_web/static/app.js` | UI state, localization, WASM calls, preview, and downloads |
| `crates/qr_forge_web/static/app.css` | Responsive layout and visual states |
| `crates/qr_forge_web/package.json` | Reproducible WASM build and Playwright commands |
| `crates/qr_forge_web/tests/browser/editor.spec.js` | Four-viewport browser behavior and privacy checks |
| `docs/CONTRACT.md` | Normative public inputs, outputs, limits, and compatibility |

## Build and Test Commands

The minimum supported Rust version is 1.85 and the workspace uses Rust edition 2024.

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
rustup target add wasm32-unknown-unknown
cargo check -p qr_forge_core --target wasm32-unknown-unknown
cargo check -p qr_forge_web --target wasm32-unknown-unknown
wasm-pack build --target web crates/qr_forge_web -- --locked

cd crates/qr_forge_web
npm ci
npx playwright install chromium
npm run test:wasm-smoke:local
```

Serve the built browser application over HTTP:

```bash
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

Do not commit `target`, `pkg`, browser trace, screenshot, or other generated output.

## Core Invariants

### QR

- Data must be non-empty.
- Correction accepts L, M, Q, or H.
- Core module size is 1 through 64; the browser limits it to 1 through 32.
- Margin is 0 through 16 modules.
- Colors are six hexadecimal digits with an optional `#` prefix.
- Wi-Fi SSID and password limits are UTF-8 byte limits: 32 and 64 respectively.
- Wi-Fi credential whitespace is meaningful and must not be trimmed.
- vCard requires a non-empty trimmed name and emits vCard 3.0 with `FN`.
- Logo ratio must be finite and within 0.05 through 0.30 inclusive.
- Logo composition is PNG-only; do not enable SVG logo download without implementing and testing equivalent output.

### Barcodes

- Code 128 accepts 1 through 128 bytes of printable ASCII 32 through 126.
- Even-length all-numeric Code 128 uses subset C; all other accepted input uses subset B.
- Code 128 has a 10-module quiet zone on both sides.
- EAN-13 accepts exactly 12 ASCII digits and calculates the check digit.
- EAN-13 has an 11-module left quiet zone and a 7-module right quiet zone.
- Barcode height is 20 through 2000; module width is 1 through 16.
- Any encoding change must preserve independent decoder coverage.

## Machine-Readable Error Codes

Keep these identifiers synchronized with `CoreError::code()` and `docs/CONTRACT.md`:

| Code | Meaning |
| ------ | --------- |
| `EMPTY_DATA` | Required data is empty |
| `INVALID_DATA` | QR encoder rejects the data |
| `INVALID_EC_LEVEL` | Unsupported QR correction level |
| `INVALID_HEX_COLOR` | Invalid six-digit color |
| `INVALID_EAN13` | EAN-13 length is not 12 bytes |
| `INVALID_EAN13_CHARS` | EAN-13 contains a non-digit |
| `INVALID_CODE128` | Code 128 contains unsupported bytes |
| `CODE128_TOO_LONG` | Code 128 exceeds 128 bytes |
| `INVALID_WIFI_SSID` | SSID is empty or exceeds 32 bytes |
| `INVALID_WIFI_PASSWORD` | Password exceeds 64 bytes |
| `INVALID_WIFI_ENCRYPTION` | Unsupported Wi-Fi encryption |
| `MISSING_VCARD_NAME` | Trimmed vCard name is empty |
| `SVG_RENDER_FAILED` | SVG rendering failed |
| `PNG_RENDER_FAILED` | PNG rendering or encoding failed |
| `LOGO_OVERLAY_FAILED` | Logo decoding or compositing failed |
| `INVALID_LOGO_RATIO` | Logo ratio is non-finite or out of range |
| `INVALID_MODULE_SIZE` | QR module size is out of range |
| `INVALID_MARGIN` | QR margin is out of range |
| `INVALID_BARCODE_HEIGHT` | Barcode height is out of range |
| `INVALID_BARCODE_MODULE_WIDTH` | Barcode module width is out of range |

Do not reuse an existing code for a different condition. Add contract documentation and tests with any new code.

## Change Requirements

- Use outcome-focused tests for valid input, invalid input, limits, and failure strategy.
- For QR or barcode encoding changes, prove the final PNG with an independent decoder rather than checking only internal bits.
- Keep user input local. New runtime network requests require an explicit product, privacy, and security decision.
- Use structured encoders and parsers rather than duplicating protocol strings in the frontend.
- Preserve stable dimensions and keyboard behavior for browser controls.
- Update `CHANGELOG.md` for user-visible behavior and compatibility changes.
- Keep edits scoped; do not refactor unrelated modules while addressing a focused change.

## Commit Language

- Write public commit subjects and bodies in English and follow Conventional Commits.
- This repository-level rule overrides any global preference for another commit-message language.

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths.
- Check console errors and warnings, runtime network requests, keyboard navigation, visible focus, accessible labels and announcements, reduced motion, overflow, overlap, QR and barcode modes, logo behavior, and both download formats.
