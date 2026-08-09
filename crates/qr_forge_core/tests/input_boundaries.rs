use qr_forge_core::{
    barcode::{BarcodeType, generate_barcode},
    qr::{QrOptions, generate_qr, wifi_qr_config},
    render::qr_to_png_with_logo,
};

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

#[test]
fn code128_has_a_ten_module_quiet_zone_on_each_side() {
    let matrix = generate_barcode("TINKORA", BarcodeType::Code128, 100, 2, BLACK, WHITE)
        .expect("generate Code 128 matrix");

    assert_eq!(matrix.bars.first(), Some(&(10, false)));
    assert_eq!(matrix.bars.last(), Some(&(10, false)));
}

#[test]
fn ean13_has_standard_quiet_zones() {
    let matrix = generate_barcode("590123412345", BarcodeType::Ean13, 100, 2, BLACK, WHITE)
        .expect("generate EAN-13 matrix");

    assert_eq!(matrix.bars.first(), Some(&(11, false)));
    assert_eq!(matrix.bars.last(), Some(&(7, false)));
    assert_eq!(matrix.total_modules, 113);
}

#[test]
fn ean13_rejects_surrounding_whitespace() {
    let error = generate_barcode(" 590123412345 ", BarcodeType::Ean13, 100, 2, BLACK, WHITE)
        .expect_err("EAN-13 input must be exact");

    assert_eq!(error.code(), "INVALID_EAN13");
}

#[test]
fn code128_rejects_more_than_128_bytes() {
    let error = generate_barcode(&"A".repeat(129), BarcodeType::Code128, 100, 2, BLACK, WHITE)
        .expect_err("oversized Code 128 input must be rejected");

    assert_eq!(error.code(), "CODE128_TOO_LONG");
}

#[test]
fn wifi_credentials_preserve_significant_whitespace() {
    let config = wifi_qr_config(" Office WiFi ", " secret ", "WPA")
        .expect("spaces are valid credential bytes");

    assert_eq!(config, "WIFI:T:WPA;S: Office WiFi ;P: secret ;;");
}

#[test]
fn invalid_logo_ratios_are_rejected_before_logo_decoding() {
    let options = QrOptions::default();
    let matrix = generate_qr("Tinkora", &options).expect("generate QR matrix");

    for ratio in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.04, 0.31] {
        let error = qr_to_png_with_logo(&matrix, &options, &[], ratio)
            .expect_err("invalid logo ratio must be rejected");
        assert_eq!(error.code(), "INVALID_LOGO_RATIO", "ratio: {ratio}");
    }
}
