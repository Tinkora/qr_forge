use crate::error::CoreError;
use barcoders::sym::{code128::Code128, ean13::EAN13};
use serde::{Deserialize, Serialize};

const CODE128_QUIET_ZONE: u32 = 10;
const EAN13_LEFT_QUIET_ZONE: u32 = 11;
const EAN13_RIGHT_QUIET_ZONE: u32 = 7;
const MAX_CODE128_BYTES: usize = 128;

/// Supported barcode symbologies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BarcodeType {
    /// Code 128 using subset B or C.
    Code128,
    /// EAN-13 with a 12-digit input and an automatically calculated check digit.
    Ean13,
}

/// A rendered barcode represented as alternating bar widths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeMatrix {
    /// Sequence of `(width_in_modules, is_dark)` pairs, including quiet zones.
    pub bars: Vec<(u32, bool)>,
    /// Total width in modules, including quiet zones.
    pub total_modules: u32,
    /// Barcode height in pixels.
    pub height: u32,
    /// Width per module in pixels.
    pub module_width: u32,
    /// Foreground RGBA color.
    pub fg_color: [u8; 4],
    /// Background RGBA color.
    pub bg_color: [u8; 4],
}

impl BarcodeMatrix {
    /// Returns the total pixel width of the barcode.
    pub fn pixel_width(&self) -> u32 {
        self.total_modules * self.module_width
    }
}

/// Generates a Code 128 or EAN-13 barcode matrix.
pub fn generate_barcode(
    data: &str,
    barcode_type: BarcodeType,
    height: u32,
    module_width: u32,
    fg_color: [u8; 4],
    bg_color: [u8; 4],
) -> Result<BarcodeMatrix, CoreError> {
    if !(20..=2000).contains(&height) {
        return Err(CoreError::InvalidBarcodeHeight);
    }
    if !(1..=16).contains(&module_width) {
        return Err(CoreError::InvalidBarcodeModuleWidth);
    }

    let (encoded, left_quiet_zone, right_quiet_zone) = match barcode_type {
        BarcodeType::Code128 => (
            encode_code128(data)?,
            CODE128_QUIET_ZONE,
            CODE128_QUIET_ZONE,
        ),
        BarcodeType::Ean13 => (
            encode_ean13(data)?,
            EAN13_LEFT_QUIET_ZONE,
            EAN13_RIGHT_QUIET_ZONE,
        ),
    };
    let bars = bits_to_bars(&encoded, left_quiet_zone, right_quiet_zone);
    let total_modules = bars.iter().map(|(width, _)| width).sum();

    Ok(BarcodeMatrix {
        bars,
        total_modules,
        height,
        module_width,
        fg_color,
        bg_color,
    })
}

fn encode_code128(data: &str) -> Result<Vec<u8>, CoreError> {
    if data.is_empty() {
        return Err(CoreError::EmptyData);
    }
    if data.len() > MAX_CODE128_BYTES {
        return Err(CoreError::Code128TooLong(data.len()));
    }
    if !data.bytes().all(|byte| (32..=126).contains(&byte)) {
        return Err(CoreError::InvalidCode128);
    }

    let use_subset_c = data.len() % 2 == 0 && data.bytes().all(|byte| byte.is_ascii_digit());
    let start = if use_subset_c { '\u{0106}' } else { '\u{0181}' };
    let mut payload = String::with_capacity(start.len_utf8() + data.len());
    payload.push(start);
    payload.push_str(data);

    Code128::new(payload)
        .map(|barcode| barcode.encode())
        .map_err(|_| CoreError::InvalidCode128)
}

fn encode_ean13(data: &str) -> Result<Vec<u8>, CoreError> {
    if data.len() != 12 {
        return Err(CoreError::InvalidEan13(data.len()));
    }
    if !data.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(CoreError::InvalidEan13Chars);
    }

    EAN13::new(data)
        .map(|barcode| barcode.encode())
        .map_err(|_| CoreError::InvalidEan13Chars)
}

fn bits_to_bars(bits: &[u8], left_quiet_zone: u32, right_quiet_zone: u32) -> Vec<(u32, bool)> {
    let mut bars = Vec::with_capacity(bits.len() / 2 + 2);
    bars.push((left_quiet_zone, false));

    if let Some((&first, rest)) = bits.split_first() {
        let mut is_dark = first == 1;
        let mut width = 1;

        for &bit in rest {
            let next_is_dark = bit == 1;
            if next_is_dark == is_dark {
                width += 1;
            } else {
                bars.push((width, is_dark));
                is_dark = next_is_dark;
                width = 1;
            }
        }
        bars.push((width, is_dark));
    }

    bars.push((right_quiet_zone, false));
    bars
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: [u8; 4] = [0, 0, 0, 255];
    const WHITE: [u8; 4] = [255, 255, 255, 255];

    #[test]
    fn generates_code128_for_printable_ascii() {
        let matrix = generate_barcode("TINKORA-128", BarcodeType::Code128, 100, 2, BLACK, WHITE)
            .expect("generate Code 128");

        assert_eq!(matrix.height, 100);
        assert_eq!(matrix.module_width, 2);
        assert!(matrix.total_modules > 20);
    }

    #[test]
    fn numeric_code128_uses_the_compact_subset_c() {
        let numeric = encode_code128("12345678").expect("encode numeric Code 128");
        let text = encode_code128("ABCDEFGH").expect("encode text Code 128");

        assert!(numeric.len() < text.len());
    }

    #[test]
    fn rejects_unencodable_code128_characters() {
        let error = encode_code128("Tinkora\n").expect_err("control characters are unsupported");

        assert_eq!(error, CoreError::InvalidCode128);
    }

    #[test]
    fn generates_a_95_module_ean13_payload() {
        let encoded = encode_ean13("590123412345").expect("encode EAN-13");

        assert_eq!(encoded.len(), 95);
    }

    #[test]
    fn validates_ean13_length_and_characters() {
        assert_eq!(encode_ean13("123").unwrap_err(), CoreError::InvalidEan13(3));
        assert_eq!(
            encode_ean13("12345678901A").unwrap_err(),
            CoreError::InvalidEan13Chars
        );
    }

    #[test]
    fn validates_barcode_dimensions() {
        let height_error =
            generate_barcode("Tinkora", BarcodeType::Code128, 10, 2, BLACK, WHITE).unwrap_err();
        let width_error =
            generate_barcode("Tinkora", BarcodeType::Code128, 100, 0, BLACK, WHITE).unwrap_err();

        assert_eq!(height_error, CoreError::InvalidBarcodeHeight);
        assert_eq!(width_error, CoreError::InvalidBarcodeModuleWidth);
    }
}
