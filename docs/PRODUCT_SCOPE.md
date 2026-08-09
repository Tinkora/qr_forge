# Product Scope

[简体中文](./PRODUCT_SCOPE.zh-CN.md)

## Product Statement

qr_forge is a focused, privacy-first utility for creating common QR codes and retail or logistics barcodes in a browser. It serves people who need a downloadable code without sending credentials, contacts, or internal URLs to a generation service.

## Users and Jobs

| User | Job to be done |
| ------ | ---------------- |
| Developer or operator | Turn a URL, text value, phone number, or email address into a downloadable QR code |
| IT support worker | Share Wi-Fi connection details without retyping a password |
| Individual or small team | Create a vCard QR code for a contact or event |
| Designer or publisher | Produce a color-controlled SVG or PNG and optionally add a PNG logo |
| Retail or logistics user | Produce a standards-based Code 128 or EAN-13 image for a supported payload |

The privacy need is concrete: Wi-Fi passwords, contact information, and private URLs should not be sent to an unrelated QR generation API merely to create an image.

## Version 0.1 Scope

### Browser application

- Static application with no project backend, account, analytics, or payload storage
- English by default with a Simplified Chinese language switch
- URL or text, Wi-Fi, vCard 3.0, phone, email, and barcode modes
- Live preview after generation and direct SVG or PNG download
- Keyboard-operable mode tabs and form controls

### QR codes

- Arbitrary non-empty text accepted by the underlying QR encoder
- L, M, Q, and H error correction
- Configurable module size, quiet zone, foreground color, and background color
- Wi-Fi payload escaping for WPA or WPA2, WEP, and open networks
- vCard 3.0 payloads with name, phone, email, and organization fields
- Optional local PNG logo overlay for PNG output

### Barcodes

- Code 128 subset B for printable ASCII and subset C for even-length numeric input
- EAN-13 from 12 input digits with an automatically generated check digit
- Standard quiet zones, configurable module width and height, and SVG or PNG output

### Developer surface

- A Rust core crate for generation, validation, and rendering
- `wasm-bindgen` functions consumed by the included browser application
- Machine-readable codes for errors represented by `CoreError`

Exact limits and output behavior are normative in [Product Contract](./CONTRACT.md).

## Explicit Non-Goals

Version 0.1 does not provide:

- A hosted generation API, server-side processing, accounts, or cloud storage
- An MCP server, Agent Skill, ChatGPT action, Codex integration, or other agent-specific protocol
- A supported command-line application
- Data Matrix, UPC, PDF417, Aztec, or other barcode formats
- Batch generation, CSV import, templates, history, or saved projects
- PDF, EPS, or print-layout output
- Animated, dynamic, expiring, tracked, or redirect-based QR codes
- Automatic logo safety guarantees or universal scanner certification
- An npm package, crates.io release, or a stable post-1.0 API commitment
- Native mobile or desktop applications

These exclusions prevent maintenance cost and product claims from growing beyond behavior that is implemented and verified.

## Privacy Boundary

The application does not intentionally transmit entered payloads. All generation and logo decoding happen in the browser. Static hosting still exposes ordinary asset-request metadata to the host, and a browser extension or modified deployment can observe page content. Downloaded output contains the encoded data.

## Scope Change Gate

A proposed capability belongs in the product only when it has:

1. A real user problem with examples that cannot be solved adequately by the current interface
2. A bounded implementation and maintenance owner
3. A privacy and security analysis
4. Outcome-focused automated tests and a realistic browser or scanner verification plan
5. English and Simplified Chinese documentation updates
6. An explicit decision about whether it changes the public contract

Open a GitHub Discussion before implementation. Planning documents and issues must label unimplemented work as proposed; they are not product capability statements.
