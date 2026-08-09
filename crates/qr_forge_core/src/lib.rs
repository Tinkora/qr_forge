pub mod barcode;
pub mod error;
pub mod qr;
pub mod render;
pub mod wasm;

pub use barcode::{BarcodeMatrix, BarcodeType, generate_barcode};
pub use error::CoreError;
pub use qr::{QrEcLevel, QrMatrix, QrOptions, generate_qr, vcard_qr_config, wifi_qr_config};
pub use render::{barcode_to_png, barcode_to_svg, qr_to_png, qr_to_svg};
