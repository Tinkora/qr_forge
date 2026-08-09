use crate::barcode::{BarcodeType, generate_barcode};
use crate::error::CoreError;
use crate::qr::{QrEcLevel, QrOptions, generate_qr, vcard_qr_config, wifi_qr_config};
use crate::render::{barcode_to_png, barcode_to_svg, qr_to_png, qr_to_png_with_logo, qr_to_svg};
use wasm_bindgen::prelude::*;

// ── Panic Hook ────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

// ── Error Conversion ──────────────────────────────────────────────────

fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

// ── QR Code: SVG ──────────────────────────────────────────────────────

/// Generate a QR code as an SVG string.
///
/// Returns a JSON object `{ svg: "...", size: number }`.
#[wasm_bindgen]
pub fn wasm_generate_qr_svg(
    data: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let ec = ec_level.parse::<QrEcLevel>().map_err(core_err)?;
    let fg = QrOptions::parse_hex_color(fg_hex).map_err(core_err)?;
    let bg = QrOptions::parse_hex_color(bg_hex).map_err(core_err)?;
    let opts = QrOptions::new(ec, module_size, margin, fg, bg).map_err(core_err)?;

    let matrix = generate_qr(data, &opts).map_err(core_err)?;
    let svg = qr_to_svg(&matrix, &opts).map_err(core_err)?;
    let pixel_size = matrix.pixel_size();

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"svg".into(), &svg.into()).ok();
    js_sys::Reflect::set(&result, &"size".into(), &pixel_size.into()).ok();

    Ok(result.into())
}

// ── QR Code: PNG ──────────────────────────────────────────────────────

/// Generate a QR code as PNG bytes (returns a Uint8Array).
#[wasm_bindgen]
pub fn wasm_generate_qr_png(
    data: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<Vec<u8>, JsValue> {
    let ec = ec_level.parse::<QrEcLevel>().map_err(core_err)?;
    let fg = QrOptions::parse_hex_color(fg_hex).map_err(core_err)?;
    let bg = QrOptions::parse_hex_color(bg_hex).map_err(core_err)?;
    let opts = QrOptions::new(ec, module_size, margin, fg, bg).map_err(core_err)?;

    let matrix = generate_qr(data, &opts).map_err(core_err)?;
    qr_to_png(&matrix, &opts).map_err(core_err)
}

// ── QR with Logo: PNG ─────────────────────────────────────────────────

/// Generate a QR code with a logo overlay as PNG bytes.
// This flat signature is part of the public JavaScript ABI exposed by wasm-bindgen.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn wasm_generate_qr_png_with_logo(
    data: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
    logo_data: &[u8],
    logo_ratio: f64,
) -> Result<Vec<u8>, JsValue> {
    let ec = ec_level.parse::<QrEcLevel>().map_err(core_err)?;
    let fg = QrOptions::parse_hex_color(fg_hex).map_err(core_err)?;
    let bg = QrOptions::parse_hex_color(bg_hex).map_err(core_err)?;
    let opts = QrOptions::new(ec, module_size, margin, fg, bg).map_err(core_err)?;

    let matrix = generate_qr(data, &opts).map_err(core_err)?;
    qr_to_png_with_logo(&matrix, &opts, logo_data, logo_ratio).map_err(core_err)
}

// ── Wi-Fi QR: SVG ─────────────────────────────────────────────────────

/// Generate a Wi-Fi network QR code as an SVG string.
// This flat signature is part of the public JavaScript ABI exposed by wasm-bindgen.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn wasm_wifi_qr_svg(
    ssid: &str,
    password: &str,
    encryption: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let wifi_data = wifi_qr_config(ssid, password, encryption).map_err(core_err)?;
    wasm_generate_qr_svg(&wifi_data, ec_level, module_size, margin, fg_hex, bg_hex)
}

/// Generate a Wi-Fi network QR code as PNG bytes.
// This flat signature is part of the public JavaScript ABI exposed by wasm-bindgen.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn wasm_wifi_qr_png(
    ssid: &str,
    password: &str,
    encryption: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<Vec<u8>, JsValue> {
    let wifi_data = wifi_qr_config(ssid, password, encryption).map_err(core_err)?;
    wasm_generate_qr_png(&wifi_data, ec_level, module_size, margin, fg_hex, bg_hex)
}

/// Build the canonical Wi-Fi QR payload without rendering it.
#[wasm_bindgen]
pub fn wasm_wifi_payload(ssid: &str, password: &str, encryption: &str) -> Result<String, JsValue> {
    wifi_qr_config(ssid, password, encryption).map_err(core_err)
}

// ── vCard QR: SVG ─────────────────────────────────────────────────────

/// Generate a vCard contact QR code as an SVG string.
// This flat signature is part of the public JavaScript ABI exposed by wasm-bindgen.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn wasm_vcard_qr_svg(
    name: &str,
    phone: &str,
    email: &str,
    org: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let vcard_data = vcard_qr_config(name, phone, email, org).map_err(core_err)?;
    wasm_generate_qr_svg(&vcard_data, ec_level, module_size, margin, fg_hex, bg_hex)
}

/// Generate a vCard contact QR code as PNG bytes.
// This flat signature is part of the public JavaScript ABI exposed by wasm-bindgen.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn wasm_vcard_qr_png(
    name: &str,
    phone: &str,
    email: &str,
    org: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<Vec<u8>, JsValue> {
    let vcard_data = vcard_qr_config(name, phone, email, org).map_err(core_err)?;
    wasm_generate_qr_png(&vcard_data, ec_level, module_size, margin, fg_hex, bg_hex)
}

/// Build the canonical vCard QR payload without rendering it.
#[wasm_bindgen]
pub fn wasm_vcard_payload(
    name: &str,
    phone: &str,
    email: &str,
    org: &str,
) -> Result<String, JsValue> {
    vcard_qr_config(name, phone, email, org).map_err(core_err)
}

// ── Phone / Email QR Helpers ─────────────────────────────────────────

/// Generate a phone number QR code (tel: URI) as SVG.
#[wasm_bindgen]
pub fn wasm_phone_qr_svg(
    phone: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let data = format!("tel:{}", phone.trim());
    wasm_generate_qr_svg(&data, ec_level, module_size, margin, fg_hex, bg_hex)
}

/// Generate an email QR code (mailto: URI) as SVG.
#[wasm_bindgen]
pub fn wasm_email_qr_svg(
    email: &str,
    ec_level: &str,
    module_size: u32,
    margin: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let data = format!("mailto:{}", email.trim());
    wasm_generate_qr_svg(&data, ec_level, module_size, margin, fg_hex, bg_hex)
}

// ── Barcode: SVG ──────────────────────────────────────────────────────

/// Generate a Code 128 or EAN-13 barcode as an SVG string.
#[wasm_bindgen]
pub fn wasm_generate_barcode_svg(
    data: &str,
    barcode_type: &str,
    height: u32,
    module_width: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<JsValue, JsValue> {
    let btype = parse_barcode_type(barcode_type)?;
    let fg = QrOptions::parse_hex_color(fg_hex).map_err(core_err)?;
    let bg = QrOptions::parse_hex_color(bg_hex).map_err(core_err)?;

    let matrix = generate_barcode(data, btype, height, module_width, fg, bg).map_err(core_err)?;
    let svg = barcode_to_svg(&matrix).map_err(core_err)?;
    let pixel_width = matrix.pixel_width();

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"svg".into(), &svg.into()).ok();
    js_sys::Reflect::set(&result, &"width".into(), &pixel_width.into()).ok();
    js_sys::Reflect::set(&result, &"height".into(), &height.into()).ok();

    Ok(result.into())
}

/// Generate a Code 128 or EAN-13 barcode as PNG bytes.
#[wasm_bindgen]
pub fn wasm_generate_barcode_png(
    data: &str,
    barcode_type: &str,
    height: u32,
    module_width: u32,
    fg_hex: &str,
    bg_hex: &str,
) -> Result<Vec<u8>, JsValue> {
    let btype = parse_barcode_type(barcode_type)?;
    let fg = QrOptions::parse_hex_color(fg_hex).map_err(core_err)?;
    let bg = QrOptions::parse_hex_color(bg_hex).map_err(core_err)?;

    let matrix = generate_barcode(data, btype, height, module_width, fg, bg).map_err(core_err)?;
    barcode_to_png(&matrix).map_err(core_err)
}

fn parse_barcode_type(s: &str) -> Result<BarcodeType, JsValue> {
    match s.to_lowercase().as_str() {
        "code128" | "code_128" | "code-128" => Ok(BarcodeType::Code128),
        "ean13" | "ean_13" | "ean-13" => Ok(BarcodeType::Ean13),
        other => Err(JsValue::from_str(&format!(
            "Unknown barcode type: {}. Use 'code128' or 'ean13'.",
            other
        ))),
    }
}
