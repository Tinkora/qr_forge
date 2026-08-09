use wasm_bindgen::prelude::*;

// ── Re-export Core Functions ──────────────────────────────────────────

/// Re-export the core WASM functions for the web bundle.
pub use qr_forge_core::wasm::{
    wasm_email_qr_svg, wasm_generate_barcode_png, wasm_generate_barcode_svg, wasm_generate_qr_png,
    wasm_generate_qr_png_with_logo, wasm_generate_qr_svg, wasm_phone_qr_svg, wasm_vcard_payload,
    wasm_vcard_qr_png, wasm_vcard_qr_svg, wasm_wifi_payload, wasm_wifi_qr_png, wasm_wifi_qr_svg,
};

/// Get library version string.
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
