use std::collections::HashSet;

use qr_forge_core::{
    barcode::{BarcodeType, generate_barcode},
    qr::{QrOptions, generate_qr},
    render::{barcode_to_png, qr_to_png},
};
use rxing::{
    BarcodeFormat, BinaryBitmap, DecodeHints, MultiFormatReader, RGBLuminanceSource, Reader,
    common::HybridBinarizer,
};

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

fn decode_png(png: &[u8], format: BarcodeFormat) -> rxing::RXingResult {
    let image = image::load_from_memory(png)
        .expect("load generated PNG")
        .to_rgb8();
    let (width, height) = image.dimensions();
    let pixels = image
        .pixels()
        .map(|pixel| (u32::from(pixel[0]) << 16) | (u32::from(pixel[1]) << 8) | u32::from(pixel[2]))
        .collect::<Vec<_>>();
    let source =
        RGBLuminanceSource::new_with_width_height_pixels(width as usize, height as usize, &pixels)
            .expect("construct scanner luminance source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    let hints = DecodeHints {
        PossibleFormats: Some(HashSet::from([format])),
        TryHarder: Some(true),
        ..DecodeHints::default()
    };

    MultiFormatReader::default()
        .decode_with_hints(&mut bitmap, &hints)
        .expect("independent scanner should decode generated PNG")
}

#[test]
fn generated_qr_decodes_with_an_independent_scanner() {
    let payload = "https://tinkora.github.io/qr_forge/";
    let options = QrOptions::default();
    let matrix = generate_qr(payload, &options).expect("generate QR matrix");
    let png = qr_to_png(&matrix, &options).expect("render QR PNG");

    let decoded = decode_png(&png, BarcodeFormat::QR_CODE);
    assert_eq!(decoded.getText(), payload);
    assert_eq!(decoded.getBarcodeFormat(), &BarcodeFormat::QR_CODE);
}

#[test]
fn generated_code128_decodes_with_an_independent_scanner() {
    let payload = "TINKORA-128";
    let matrix = generate_barcode(payload, BarcodeType::Code128, 160, 3, BLACK, WHITE)
        .expect("generate Code 128 matrix");
    let png = barcode_to_png(&matrix).expect("render Code 128 PNG");

    let decoded = decode_png(&png, BarcodeFormat::CODE_128);
    assert_eq!(decoded.getText(), payload);
    assert_eq!(decoded.getBarcodeFormat(), &BarcodeFormat::CODE_128);
}

#[test]
fn generated_ean13_decodes_with_an_independent_scanner() {
    let matrix = generate_barcode("590123412345", BarcodeType::Ean13, 160, 3, BLACK, WHITE)
        .expect("generate EAN-13 matrix");
    let png = barcode_to_png(&matrix).expect("render EAN-13 PNG");

    let decoded = decode_png(&png, BarcodeFormat::EAN_13);
    assert_eq!(decoded.getText(), "5901234123457");
    assert_eq!(decoded.getBarcodeFormat(), &BarcodeFormat::EAN_13);
}
