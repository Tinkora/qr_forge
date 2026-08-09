use crate::error::CoreError;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// ── Error Correction Level ────────────────────────────────────────────

/// QR code error correction level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum QrEcLevel {
    /// ~7% recovery
    L,
    /// ~15% recovery
    M,
    /// ~25% recovery
    Q,
    /// ~30% recovery
    H,
}

impl FromStr for QrEcLevel {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            "Q" => Ok(Self::Q),
            "H" => Ok(Self::H),
            other => Err(CoreError::InvalidEcLevel(other.to_string())),
        }
    }
}

impl QrEcLevel {
    /// Convert to the `qrcode` crate's `EcLevel`.
    fn to_qrcode_ec(self) -> qrcode::EcLevel {
        match self {
            Self::L => qrcode::EcLevel::L,
            Self::M => qrcode::EcLevel::M,
            Self::Q => qrcode::EcLevel::Q,
            Self::H => qrcode::EcLevel::H,
        }
    }
}

// ── QR Options ────────────────────────────────────────────────────────

/// Configuration for QR code generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrOptions {
    /// Error correction level.
    pub ec_level: QrEcLevel,
    /// Pixel size per module (1..64).
    pub module_size: u32,
    /// Quiet zone margin in modules (0..16).
    pub margin: u32,
    /// Foreground RGBA color.
    pub fg_color: [u8; 4],
    /// Background RGBA color.
    pub bg_color: [u8; 4],
}

impl Default for QrOptions {
    fn default() -> Self {
        Self {
            ec_level: QrEcLevel::M,
            module_size: 8,
            margin: 4,
            fg_color: [0, 0, 0, 255],
            bg_color: [255, 255, 255, 255],
        }
    }
}

impl QrOptions {
    /// Create new options with validation.
    pub fn new(
        ec_level: QrEcLevel,
        module_size: u32,
        margin: u32,
        fg_color: [u8; 4],
        bg_color: [u8; 4],
    ) -> Result<Self, CoreError> {
        if !(1..=64).contains(&module_size) {
            return Err(CoreError::InvalidModuleSize);
        }
        if margin > 16 {
            return Err(CoreError::InvalidMargin);
        }
        Ok(Self {
            ec_level,
            module_size,
            margin,
            fg_color,
            bg_color,
        })
    }

    /// Parse a hex color string like "#FF8800" or "FF8800" into RGBA bytes.
    pub fn parse_hex_color(hex: &str) -> Result<[u8; 4], CoreError> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(CoreError::InvalidHexColor(hex.to_string()));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| CoreError::InvalidHexColor(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| CoreError::InvalidHexColor(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| CoreError::InvalidHexColor(hex.to_string()))?;
        Ok([r, g, b, 255])
    }

    /// Format RGBA as hex string "#RRGGBB".
    pub fn color_to_hex(color: &[u8; 4]) -> String {
        format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
    }
}

// ── QR Matrix ─────────────────────────────────────────────────────────

/// The QR code module matrix produced by `generate_qr`.
///
/// Each `bool` represents one module: `true` = dark (foreground), `false` = light (background).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrMatrix {
    /// Number of modules per side (the QR code is square).
    pub size: u32,
    /// Row-major module data: `modules[row * size + col]`.
    pub modules: Vec<bool>,
    /// The options used to generate this matrix.
    pub options: QrOptions,
}

impl QrMatrix {
    /// Get the total pixel width/height of the rendered image.
    pub fn pixel_size(&self) -> u32 {
        let modules = self.size + 2 * self.options.margin;
        modules * self.options.module_size
    }

    /// Check if the module at (row, col) is dark.
    #[inline]
    pub fn is_dark(&self, row: u32, col: u32) -> bool {
        self.modules
            .get((row as usize * self.size as usize) + col as usize)
            .copied()
            .unwrap_or(false)
    }
}

// ── QR Generation ─────────────────────────────────────────────────────

/// Generate a QR code matrix from arbitrary data.
///
/// # Errors
///
/// Returns `CoreError::EmptyData` if `data` is empty.
pub fn generate_qr(data: &str, options: &QrOptions) -> Result<QrMatrix, CoreError> {
    if data.is_empty() {
        return Err(CoreError::EmptyData);
    }

    let code = QrCode::with_error_correction_level(data, options.ec_level.to_qrcode_ec())
        .map_err(|_e| CoreError::InvalidData)?;

    let size = code.width() as u32;
    let mut modules = Vec::with_capacity((size * size) as usize);

    for row in 0..size {
        for col in 0..size {
            modules.push(code[(col as usize, row as usize)] == qrcode::Color::Dark);
        }
    }

    Ok(QrMatrix {
        size,
        modules,
        options: options.clone(),
    })
}

// ── Wi-Fi QR Config ───────────────────────────────────────────────────

/// Escape special characters for Wi-Fi QR config strings.
fn escape_wifi(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace(':', "\\:")
        .replace('"', "\\\"")
}

/// Build a Wi-Fi network configuration string for QR encoding.
///
/// Format: `WIFI:S:<ssid>;T:<encryption>;P:<password>;;`
///
/// # Errors
///
/// Returns error if SSID is empty or too long, password too long,
/// or encryption type is invalid.
pub fn wifi_qr_config(ssid: &str, password: &str, encryption: &str) -> Result<String, CoreError> {
    if ssid.is_empty() || ssid.len() > 32 {
        return Err(CoreError::InvalidWifiSsid);
    }
    if password.len() > 64 {
        return Err(CoreError::InvalidWifiPassword);
    }

    let enc = match encryption.to_uppercase().as_str() {
        "WPA" | "WPA2" => "WPA",
        "WEP" => "WEP",
        "NOPASS" | "" => "nopass",
        _other => return Err(CoreError::InvalidWifiEncryption),
    };

    let escaped_ssid = escape_wifi(ssid);
    let escaped_pw = escape_wifi(password);

    // Per Wi-Fi QR spec: WIFI:S:<ssid>;T:<enc>;P:<pw>;H:false;;
    if enc == "nopass" {
        Ok(format!("WIFI:S:{};T:nopass;;", escaped_ssid))
    } else {
        Ok(format!(
            "WIFI:T:{};S:{};P:{};;",
            enc, escaped_ssid, escaped_pw
        ))
    }
}

// ── vCard QR Config ───────────────────────────────────────────────────

/// Escape vCard special characters.
fn escape_vcard(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
}

/// Build a vCard 3.0 string for QR encoding.
///
/// # Errors
///
/// Returns error if `name` is empty.
pub fn vcard_qr_config(
    name: &str,
    phone: &str,
    email: &str,
    org: &str,
) -> Result<String, CoreError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CoreError::MissingVcardName);
    }

    let mut vcard = String::from("BEGIN:VCARD\nVERSION:3.0\n");
    vcard.push_str(&format!("FN:{}\n", escape_vcard(name)));

    if !phone.trim().is_empty() {
        vcard.push_str(&format!("TEL:{}\n", escape_vcard(phone.trim())));
    }
    if !email.trim().is_empty() {
        vcard.push_str(&format!("EMAIL:{}\n", escape_vcard(email.trim())));
    }
    if !org.trim().is_empty() {
        vcard.push_str(&format!("ORG:{}\n", escape_vcard(org.trim())));
    }

    vcard.push_str("END:VCARD");
    Ok(vcard)
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_qr_url() {
        let opts = QrOptions::default();
        let matrix = generate_qr("https://example.com", &opts).unwrap();
        assert!(matrix.size >= 21); // Minimum QR version 1
        assert_eq!(matrix.modules.len(), (matrix.size * matrix.size) as usize);
    }

    #[test]
    fn test_generate_qr_empty_fails() {
        let opts = QrOptions::default();
        let err = generate_qr("", &opts).unwrap_err();
        assert_eq!(err, CoreError::EmptyData);
    }

    #[test]
    fn test_qr_ec_level_from_str() {
        assert_eq!("L".parse::<QrEcLevel>().unwrap(), QrEcLevel::L);
        assert_eq!("m".parse::<QrEcLevel>().unwrap(), QrEcLevel::M);
        assert_eq!("H".parse::<QrEcLevel>().unwrap(), QrEcLevel::H);
        assert!("X".parse::<QrEcLevel>().is_err());
    }

    #[test]
    fn test_wifi_qr_config_wpa() {
        let cfg = wifi_qr_config("MyWiFi", "secret123", "WPA").unwrap();
        assert!(cfg.contains("WIFI:"));
        assert!(cfg.contains("MyWiFi"));
        assert!(cfg.contains("secret123"));
        assert!(cfg.contains("T:WPA"));
    }

    #[test]
    fn test_wifi_qr_config_nopass() {
        let cfg = wifi_qr_config("OpenNet", "", "nopass").unwrap();
        assert!(cfg.contains("T:nopass"));
        assert!(!cfg.contains("P:"));
    }

    #[test]
    fn test_wifi_qr_config_invalid_ssid() {
        let err = wifi_qr_config("", "pw", "WPA").unwrap_err();
        assert_eq!(err, CoreError::InvalidWifiSsid);
    }

    #[test]
    fn test_vcard_qr_config() {
        let cfg =
            vcard_qr_config("John Doe", "1234567890", "john@example.com", "ACME Corp").unwrap();
        assert!(cfg.contains("BEGIN:VCARD"));
        assert!(cfg.contains("FN:John Doe"));
        assert!(cfg.contains("TEL:1234567890"));
        assert!(cfg.contains("EMAIL:john@example.com"));
        assert!(cfg.contains("ORG:ACME Corp"));
        assert!(cfg.contains("END:VCARD"));
    }

    #[test]
    fn test_vcard_missing_name() {
        let err = vcard_qr_config("", "", "", "").unwrap_err();
        assert_eq!(err, CoreError::MissingVcardName);
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(
            QrOptions::parse_hex_color("#FF8800").unwrap(),
            [255, 136, 0, 255]
        );
        assert_eq!(
            QrOptions::parse_hex_color("00AAFF").unwrap(),
            [0, 170, 255, 255]
        );
        assert!(QrOptions::parse_hex_color("#GGG").is_err());
    }

    #[test]
    fn test_qr_pixel_size() {
        let opts = QrOptions {
            module_size: 4,
            margin: 2,
            ..Default::default()
        };
        let matrix = generate_qr("test", &opts).unwrap();
        let px = (matrix.size + 4) * 4;
        assert_eq!(matrix.pixel_size(), px);
    }

    #[test]
    fn test_module_size_validation() {
        assert!(QrOptions::new(QrEcLevel::M, 0, 4, [0; 4], [255; 4]).is_err());
        assert!(QrOptions::new(QrEcLevel::M, 65, 4, [0; 4], [255; 4]).is_err());
        assert!(QrOptions::new(QrEcLevel::M, 8, 17, [0; 4], [255; 4]).is_err());
    }
}
