use thiserror::Error;

/// Stable error type for QR and barcode generation.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Input data is empty")]
    EmptyData,

    #[error("Input data contains invalid characters")]
    InvalidData,

    #[error("Invalid error correction level: {0}")]
    InvalidEcLevel(String),

    #[error("Invalid hex color: {0}")]
    InvalidHexColor(String),

    #[error("EAN-13 input must be exactly 12 digits, got {0}")]
    InvalidEan13(usize),

    #[error("EAN-13 input must contain only digits")]
    InvalidEan13Chars,

    #[error("Code 128 input contains unencodable characters")]
    InvalidCode128,

    #[error("Code 128 input must not exceed 128 bytes, got {0}")]
    Code128TooLong(usize),

    #[error("Wi-Fi SSID must not be empty and ≤ 32 characters")]
    InvalidWifiSsid,

    #[error("Wi-Fi password must be ≤ 64 characters")]
    InvalidWifiPassword,

    #[error("Wi-Fi encryption must be WPA, WEP, or nopass")]
    InvalidWifiEncryption,

    #[error("vCard requires at least a name")]
    MissingVcardName,

    #[error("SVG rendering failed: {0}")]
    SvgRenderFailed(String),

    #[error("PNG rendering failed: {0}")]
    PngRenderFailed(String),

    #[error("Logo overlay failed: {0}")]
    LogoOverlayFailed(String),

    #[error("Logo ratio must be finite and between 0.05 and 0.30")]
    InvalidLogoRatio,

    #[error("Module size must be between 1 and 64")]
    InvalidModuleSize,

    #[error("Margin must be between 0 and 16")]
    InvalidMargin,

    #[error("Barcode height must be between 20 and 2000")]
    InvalidBarcodeHeight,

    #[error("Barcode module width must be between 1 and 16")]
    InvalidBarcodeModuleWidth,
}

impl CoreError {
    /// Returns a stable machine error code for Rust, WebAssembly, and browser consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyData => "EMPTY_DATA",
            Self::InvalidData => "INVALID_DATA",
            Self::InvalidEcLevel(_) => "INVALID_EC_LEVEL",
            Self::InvalidHexColor(_) => "INVALID_HEX_COLOR",
            Self::InvalidEan13(_) => "INVALID_EAN13",
            Self::InvalidEan13Chars => "INVALID_EAN13_CHARS",
            Self::InvalidCode128 => "INVALID_CODE128",
            Self::Code128TooLong(_) => "CODE128_TOO_LONG",
            Self::InvalidWifiSsid => "INVALID_WIFI_SSID",
            Self::InvalidWifiPassword => "INVALID_WIFI_PASSWORD",
            Self::InvalidWifiEncryption => "INVALID_WIFI_ENCRYPTION",
            Self::MissingVcardName => "MISSING_VCARD_NAME",
            Self::SvgRenderFailed(_) => "SVG_RENDER_FAILED",
            Self::PngRenderFailed(_) => "PNG_RENDER_FAILED",
            Self::LogoOverlayFailed(_) => "LOGO_OVERLAY_FAILED",
            Self::InvalidLogoRatio => "INVALID_LOGO_RATIO",
            Self::InvalidModuleSize => "INVALID_MODULE_SIZE",
            Self::InvalidMargin => "INVALID_MARGIN",
            Self::InvalidBarcodeHeight => "INVALID_BARCODE_HEIGHT",
            Self::InvalidBarcodeModuleWidth => "INVALID_BARCODE_MODULE_WIDTH",
        }
    }
}
