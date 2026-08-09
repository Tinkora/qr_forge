# Product Contract

[简体中文](./CONTRACT.zh-CN.md)

This document defines the implemented input, output, and error behavior for qr_forge 0.1.x. Product descriptions should not promise behavior beyond this contract. The source and tests remain authoritative if a documentation defect is found.

## Supported Interfaces

1. The static browser application in `crates/qr_forge_web/static`
2. The public Rust modules and re-exports in `qr_forge_core`
3. The JavaScript functions exported through `wasm-bindgen` by `qr_forge_web`

The project does not currently publish a hosted generation API, CLI, MCP interface, npm package, or crates.io package.

## Common QR Contract

| Input | Accepted value | Default in Rust core | Browser UI range |
| ------- | ---------------- | ---------------------- | ------------------ |
| Data | Non-empty UTF-8 text accepted by `qrcode` | None | Non-empty |
| Error correction | `L`, `M`, `Q`, or `H`, case-insensitive in Rust/WASM | `M` | `L`, `M`, `Q`, or `H`; initial selection is `H` |
| Module size | Integer from 1 through 64 pixels | `8` | 1 through 32 |
| Margin | Integer from 0 through 16 modules | `4` | 0 through 16 |
| Foreground | Six hexadecimal digits, with an optional leading `#` | `#000000` | Browser color input |
| Background | Six hexadecimal digits, with an optional leading `#` | `#FFFFFF` | Browser color input |

SVG output is a square SVG string made from background and module `<rect>` elements. PNG output is an RGBA PNG byte sequence. The output pixel size is `(matrix modules + 2 * margin) * module size` on each side.

The user controls the margin. A margin of zero is valid but may reduce scanner compatibility. Consumers are responsible for choosing sufficient contrast and testing critical output.

## Payload Modes

### URL or text

The value is encoded without URL normalization or network validation. Whitespace is significant. Empty input is rejected.

### Wi-Fi

`wifi_qr_config` and its WASM wrappers enforce:

- SSID is non-empty and at most 32 UTF-8 bytes
- Password is at most 64 UTF-8 bytes
- Encryption accepts `WPA`, `WPA2`, `WEP`, `NOPASS`, or an empty string, case-insensitively
- `WPA2` is emitted as `WPA`; `NOPASS` and empty encryption are emitted as `nopass`
- Backslash, semicolon, comma, colon, and double quote are escaped
- Leading and trailing SSID and password whitespace is preserved

Protected networks produce `WIFI:T:<type>;S:<ssid>;P:<password>;;`. Open networks produce `WIFI:S:<ssid>;T:nopass;;`.

The browser UI offers WPA or WPA2, WEP, and open-network choices. It does not connect to the network or validate the credentials with an access point.

### vCard

`vcard_qr_config` emits vCard 3.0 text with an `FN` field and optional `TEL`, `EMAIL`, and `ORG` fields. The name is required after trimming. All fields are trimmed; backslash, semicolon, comma, and newline characters are escaped. The function does not validate phone or email syntax.

### Phone and email

The SVG helpers trim the input and prefix it with `tel:` or `mailto:`. They do not normalize or validate the value. The browser UI requires a non-empty value before generation.

## Barcode Contract

| Type | Input | Encoding | Quiet zone |
| --- | --- | --- | --- |
| Code 128 | 1 to 128 bytes, each ASCII 32 through 126 | Subset C for even-length all-numeric input; subset B otherwise | 10 modules on each side |
| EAN-13 | Exactly 12 ASCII digits | Check digit calculated by the encoder; 95 encoded data modules | 11 modules left and 7 modules right |

Barcode height must be 20 through 2000 pixels. Module width must be 1 through 16 pixels. SVG and PNG outputs contain bars and background only; no human-readable text is drawn. Pixel width includes the quiet zones.

Code 128 does not support control characters, non-ASCII text, subset A, or adaptive switching within one payload. EAN-13 input must not contain spaces or the thirteenth check digit.

## Logo Overlay Contract

- Logo input must decode as PNG in the shipped build.
- The ratio must be finite and from `0.05` through `0.30`, inclusive.
- The logo is resized to a square and centered over the QR data area.
- A two-pixel white padding area is placed around the logo.
- The browser accepts PNG files up to 2 MiB and automatically selects H correction.
- A logo is included only in PNG output. The browser disables SVG download while a logo is active so it cannot offer an output that silently omits the logo.

Logo placement can obscure modules. Successful generation is not a guarantee that every camera or scanner will decode the result.

## Rust API

The primary public Rust surface is:

- `generate_qr`, `QrOptions`, `QrEcLevel`, and `QrMatrix`
- `wifi_qr_config` and `vcard_qr_config`
- `generate_barcode`, `BarcodeType`, and `BarcodeMatrix`
- `qr_to_svg`, `qr_to_png`, `barcode_to_svg`, and `barcode_to_png`
- `render::qr_to_png_with_logo`
- `CoreError`

Matrix structures are serializable but should not be treated as a durable storage format before 1.0.

## WebAssembly API

All color parameters below are strings and all sizes are unsigned integers unless noted.

| Function | Result |
| ---------- | -------- |
| `wasm_generate_qr_svg(data, ec, module_size, margin, fg, bg)` | Object with `svg` and numeric `size` |
| `wasm_generate_qr_png(data, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_generate_qr_png_with_logo(data, ec, module_size, margin, fg, bg, logo, ratio)` | PNG `Uint8Array` |
| `wasm_wifi_qr_svg(ssid, password, encryption, ec, module_size, margin, fg, bg)` | Object with `svg` and `size` |
| `wasm_wifi_qr_png(ssid, password, encryption, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_wifi_payload(ssid, password, encryption)` | Wi-Fi payload string |
| `wasm_vcard_qr_svg(name, phone, email, org, ec, module_size, margin, fg, bg)` | Object with `svg` and `size` |
| `wasm_vcard_qr_png(name, phone, email, org, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_vcard_payload(name, phone, email, org)` | vCard payload string |
| `wasm_phone_qr_svg(phone, ec, module_size, margin, fg, bg)` | Object with `svg` and `size` |
| `wasm_email_qr_svg(email, ec, module_size, margin, fg, bg)` | Object with `svg` and `size` |
| `wasm_generate_barcode_svg(data, type, height, module_width, fg, bg)` | Object with `svg`, numeric `width`, and numeric `height` |
| `wasm_generate_barcode_png(data, type, height, module_width, fg, bg)` | PNG `Uint8Array` |
| `get_version()` | Package version string |

Barcode type accepts `code128`, `code_128`, `code-128`, `ean13`, `ean_13`, or `ean-13`, case-insensitively. An unknown barcode type currently throws a plain JavaScript string; errors derived from `CoreError` throw an object containing `code` and `message`.

## Machine-Readable Error Codes

| Code | Condition |
| ------ | ----------- |
| `EMPTY_DATA` | Required data is empty |
| `INVALID_DATA` | The QR encoder rejects the data |
| `INVALID_EC_LEVEL` | Error correction is not L, M, Q, or H |
| `INVALID_HEX_COLOR` | A color is not six hexadecimal digits |
| `INVALID_EAN13` | EAN-13 input length is not 12 bytes |
| `INVALID_EAN13_CHARS` | EAN-13 input contains a non-digit |
| `INVALID_CODE128` | Code 128 input is empty only through `EMPTY_DATA`, or otherwise contains unsupported bytes |
| `CODE128_TOO_LONG` | Code 128 input exceeds 128 bytes |
| `INVALID_WIFI_SSID` | SSID is empty or exceeds 32 bytes |
| `INVALID_WIFI_PASSWORD` | Password exceeds 64 bytes |
| `INVALID_WIFI_ENCRYPTION` | Encryption is unsupported |
| `MISSING_VCARD_NAME` | Trimmed vCard name is empty |
| `SVG_RENDER_FAILED` | SVG rendering fails |
| `PNG_RENDER_FAILED` | PNG rendering or encoding fails |
| `LOGO_OVERLAY_FAILED` | Logo decoding, sizing, or compositing fails |
| `INVALID_LOGO_RATIO` | Logo ratio is non-finite or outside 0.05 through 0.30 |
| `INVALID_MODULE_SIZE` | QR module size is outside 1 through 64 |
| `INVALID_MARGIN` | QR margin exceeds 16 |
| `INVALID_BARCODE_HEIGHT` | Barcode height is outside 20 through 2000 |
| `INVALID_BARCODE_MODULE_WIDTH` | Barcode module width is outside 1 through 16 |

Error messages are human-readable diagnostic text and may be clarified. Consumers should branch on `code`, not the English message.

## Compatibility Policy

Version 0.1 is pre-1.0. Rust types, WASM signatures, generated markup, and UI structure may change in a minor release when the change is documented in [CHANGELOG.md](../CHANGELOG.md). Maintainers should avoid changing an existing error code's meaning within 0.1.x and should provide a migration note for intentional contract changes.
