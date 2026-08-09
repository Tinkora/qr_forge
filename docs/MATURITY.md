# Maturity and Compatibility

[简体中文](./MATURITY.zh-CN.md)

## Current Status

qr_forge 0.1 is an early public release. Its core generation paths are implemented and tested, but the project is not yet a stable 1.0 API or a universally certified barcode production system.

| Area | Status | Evidence and limitation |
| ------ | -------- | ------------------------- |
| QR generation | Usable | Unit tests cover generation and limits; PNG output is decoded by an independent scanner library |
| Code 128 | Usable within contract | Printable ASCII and size limits are tested; generated PNG is independently decoded |
| EAN-13 | Usable within contract | Check digit, exact input, standard quiet zones, and independent decoding are tested |
| SVG and PNG rendering | Usable | Native tests verify both formats; output still requires real-world contrast and scanner checks |
| PNG logo overlay | Experimental | Input and ratio limits are tested; scannability depends on payload, logo, size, contrast, and scanner |
| Browser application | Beta | Verified in Chromium across 375, 768, 1024, and 1440 pixel widths; broader browser automation is not yet a compatibility guarantee |
| Rust API | Pre-1.0 | Public and tested, but types and signatures can change before 1.0 |
| WASM API | Pre-1.0 | Used by the included app; no separately versioned npm distribution |
| Accessibility | Beta | Keyboard, focus, labels, and layout are checked; no formal third-party audit |
| Security | Best effort | Browser-local architecture and automated dependency or code checks reduce risk; no external audit |

## Verified Invariants

The repository includes tests intended to prove these behaviors:

- QR, Code 128, and EAN-13 PNG output can be decoded by the independent `rxing` test dependency.
- Code 128 has a 10-module quiet zone on each side.
- EAN-13 has an 11-module left quiet zone and a 7-module right quiet zone.
- EAN-13 rejects whitespace and non-digit input instead of silently normalizing it.
- Code 128 rejects input beyond 128 bytes.
- Wi-Fi credentials preserve leading and trailing whitespace and escape delimiters.
- Non-finite and out-of-range logo ratios are rejected before image decoding.

Passing these tests does not certify every printed size, material, camera, scanner, lighting condition, or color combination.

## Compatibility Baseline

- Minimum supported Rust version: 1.85
- Rust edition: 2024
- Web build target: `wasm32-unknown-unknown`
- Web runtime: a modern browser with WebAssembly, JavaScript modules, Blob downloads, and standard DOM APIs
- Primary hosted environment: GitHub Pages
- Application languages: English and Simplified Chinese

The repository must continue to compile with Rust 1.85. CI on newer stable Rust does not substitute for an MSRV check.

## Known Limitations

- The browser UI accepts PNG logos only, up to 2 MiB.
- SVG download is intentionally unavailable while a logo is active because logo composition is PNG-only.
- Code 128 supports subset B or the all-numeric subset C case, not subset A or mixed subset switching.
- EAN-13 accepts only the first 12 digits; users cannot supply an existing thirteenth check digit.
- vCard output has a small field set and does not perform phone or email validation.
- Phone and email modes add URI prefixes but do not validate destination syntax.
- The application has no offline service worker, saved projects, history, or batch workflow.
- The project has no CLI, MCP integration, Agent Skill, hosted API, npm release, or crates.io release.
- English and Simplified Chinese are the only application and project-documentation languages maintained as first-class versions.

## Versioning Policy

The project uses Semantic Versioning, but versions below 1.0 may make contract changes in a minor release. Every user-visible or API-visible change must be recorded in the changelog. A patch release should remain backward compatible within its minor line unless it fixes unsafe or incorrect behavior that cannot reasonably be preserved.

The criteria for 1.0 include:

1. At least one full public release cycle with documented upgrade experience
2. A deliberate Rust and WASM API review
3. Reproducible release artifacts and enforced supply-chain checks
4. A maintained multi-browser compatibility matrix
5. Stable public error semantics and migration policy
6. Evidence of sustained real-world use and maintainership capacity
