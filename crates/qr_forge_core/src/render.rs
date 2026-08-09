use crate::barcode::BarcodeMatrix;
use crate::error::CoreError;
use crate::qr::{QrMatrix, QrOptions};
use image::{ImageBuffer, Rgba, RgbaImage};

/// Format an RGBA color as an SVG fill string: `rgb(r,g,b)` or `rgba(r,g,b,a)`.
fn rgba_to_svg_fill(color: &[u8; 4]) -> String {
    if color[3] == 255 {
        format!("rgb({},{},{})", color[0], color[1], color[2])
    } else {
        format!(
            "rgba({},{},{},{:.3})",
            color[0],
            color[1],
            color[2],
            color[3] as f64 / 255.0
        )
    }
}

// ── QR to SVG ─────────────────────────────────────────────────────────

/// Render a QR code matrix to an SVG string.
///
/// Generates a clean, scalable SVG with `<rect>` elements for each module.
/// Uses the module size and margin from the options.
pub fn qr_to_svg(matrix: &QrMatrix, options: &QrOptions) -> Result<String, CoreError> {
    let size = matrix.size;
    let ms = options.module_size;
    let margin = options.margin;
    let total = size + 2 * margin;
    let total_px = total * ms;

    let fg = rgba_to_svg_fill(&options.fg_color);
    let bg = rgba_to_svg_fill(&options.bg_color);

    let mut svg = String::with_capacity(4096);

    // SVG header
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{px}" height="{px}" viewBox="0 0 {px} {px}">"#,
        px = total_px
    ));
    svg.push('\n');

    // Background rect
    svg.push_str(&format!(
        r#"  <rect width="{px}" height="{px}" fill="{bg}" />"#,
        px = total_px,
        bg = bg
    ));
    svg.push('\n');

    // Draw dark modules as rects — run-length encoded horizontally for efficiency
    for row in 0..size {
        let mut col = 0u32;
        while col < size {
            if matrix.is_dark(row, col) {
                // Find run of dark modules
                let run_start = col;
                while col < size && matrix.is_dark(row, col) {
                    col += 1;
                }
                let run_len = col - run_start;
                let rx = (margin + run_start) * ms;
                let ry = (margin + row) * ms;
                svg.push_str(&format!(
                    r#"  <rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{fg}" />"#,
                    x = rx,
                    y = ry,
                    w = run_len * ms,
                    h = ms,
                    fg = fg
                ));
                svg.push('\n');
            } else {
                col += 1;
            }
        }
    }

    svg.push_str("</svg>\n");

    Ok(svg)
}

// ── QR to PNG ─────────────────────────────────────────────────────────

/// Render a QR code matrix to PNG bytes.
///
/// Uses the `image` crate to create an RGBA image buffer.
pub fn qr_to_png(matrix: &QrMatrix, options: &QrOptions) -> Result<Vec<u8>, CoreError> {
    let size = matrix.size;
    let ms = options.module_size;
    let margin = options.margin;
    let total_px = matrix.pixel_size();

    let fg = Rgba(options.fg_color);
    let bg = Rgba(options.bg_color);

    let mut img: RgbaImage = ImageBuffer::from_pixel(total_px, total_px, bg);

    for row in 0..size {
        for col in 0..size {
            if matrix.is_dark(row, col) {
                let px = (margin + col) * ms;
                let py = (margin + row) * ms;
                // Fill the module area
                for dy in 0..ms {
                    for dx in 0..ms {
                        if px + dx < total_px && py + dy < total_px {
                            img.put_pixel(px + dx, py + dy, fg);
                        }
                    }
                }
            }
        }
    }

    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| CoreError::PngRenderFailed(e.to_string()))?;

    Ok(buf)
}

// ── QR with Logo Overlay ──────────────────────────────────────────────

/// Overlay a logo image (PNG bytes) at the center of a QR code, returning new PNG bytes.
///
/// The logo is scaled to approximately `logo_ratio` of the QR code's data area.
/// A white padding ring is added around the logo to ensure scannability.
pub fn qr_to_png_with_logo(
    matrix: &QrMatrix,
    options: &QrOptions,
    logo_data: &[u8],
    logo_ratio: f64,
) -> Result<Vec<u8>, CoreError> {
    if !(0.05..=0.30).contains(&logo_ratio) {
        return Err(CoreError::InvalidLogoRatio);
    }

    // Generate base QR PNG
    let qr_png = qr_to_png(matrix, options)?;

    // Load the logo
    let logo_img = image::load_from_memory(logo_data)
        .map_err(|e| CoreError::LogoOverlayFailed(format!("Failed to load logo: {e}")))?;

    let logo_rgba = logo_img.to_rgba8();

    // Load the QR image back
    let mut qr_img = image::load_from_memory(&qr_png)
        .map_err(|e| CoreError::PngRenderFailed(e.to_string()))?
        .to_rgba8();

    let data_area = (matrix.size * options.module_size) as f64;
    let logo_size = (data_area * logo_ratio) as u32;

    if logo_size < 8 {
        return Err(CoreError::LogoOverlayFailed(
            "Logo size too small".to_string(),
        ));
    }

    // Resize logo to fit
    let logo_resized = image::imageops::resize(
        &logo_rgba,
        logo_size,
        logo_size,
        image::imageops::FilterType::Lanczos3,
    );

    // Calculate position (center)
    let total = matrix.pixel_size();
    let offset_x = (total - logo_size) / 2;
    let offset_y = (total - logo_size) / 2;

    // White padding ring (2px)
    let padding = 2u32;
    let bg = Rgba([255, 255, 255, 255]);

    if offset_x > padding && offset_y > padding {
        for dy in 0..logo_size + 2 * padding {
            for dx in 0..logo_size + 2 * padding {
                let px = offset_x - padding + dx;
                let py = offset_y - padding + dy;
                if px < total && py < total {
                    qr_img.put_pixel(px, py, bg);
                }
            }
        }
    }

    // Composite logo over QR
    for dy in 0..logo_size {
        for dx in 0..logo_size {
            let logo_pixel = logo_resized.get_pixel(dx, dy);
            let px = offset_x + dx;
            let py = offset_y + dy;
            if px < total && py < total {
                // Alpha blend
                if logo_pixel[3] > 0 {
                    qr_img.put_pixel(px, py, *logo_pixel);
                }
            }
        }
    }

    let mut buf = Vec::new();
    qr_img
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| CoreError::PngRenderFailed(e.to_string()))?;

    Ok(buf)
}

// ── Barcode to SVG ────────────────────────────────────────────────────

/// Render a barcode matrix to an SVG string.
pub fn barcode_to_svg(matrix: &BarcodeMatrix) -> Result<String, CoreError> {
    let width = matrix.pixel_width();
    let height = matrix.height;
    let mw = matrix.module_width;

    let fg = rgba_to_svg_fill(&matrix.fg_color);
    let bg = rgba_to_svg_fill(&matrix.bg_color);

    let mut svg = String::with_capacity(2048);

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#,
        w = width,
        h = height
    ));
    svg.push('\n');

    // Background
    svg.push_str(&format!(
        r#"  <rect width="{w}" height="{h}" fill="{bg}" />"#,
        w = width,
        h = height,
        bg = bg
    ));
    svg.push('\n');

    // Draw bars
    let mut x_pos = 0u32;
    for &(bar_width_modules, is_dark) in &matrix.bars {
        let bar_width = bar_width_modules * mw;
        if is_dark {
            svg.push_str(&format!(
                r#"  <rect x="{x}" y="0" width="{w}" height="{h}" fill="{fg}" />"#,
                x = x_pos,
                w = bar_width,
                h = height,
                fg = fg
            ));
            svg.push('\n');
        }
        x_pos += bar_width;
    }

    svg.push_str("</svg>\n");

    Ok(svg)
}

// ── Barcode to PNG ────────────────────────────────────────────────────

/// Render a barcode matrix to PNG bytes.
pub fn barcode_to_png(matrix: &BarcodeMatrix) -> Result<Vec<u8>, CoreError> {
    let width = matrix.pixel_width();
    let height = matrix.height;
    let mw = matrix.module_width;

    let fg = Rgba(matrix.fg_color);
    let bg = Rgba(matrix.bg_color);

    let mut img: RgbaImage = ImageBuffer::from_pixel(width, height, bg);

    let mut x_pos = 0u32;
    for &(bar_width_modules, is_dark) in &matrix.bars {
        let bar_width = bar_width_modules * mw;
        if is_dark {
            for dx in 0..bar_width {
                for dy in 0..height {
                    if x_pos + dx < width {
                        img.put_pixel(x_pos + dx, dy, fg);
                    }
                }
            }
        }
        x_pos += bar_width;
    }

    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| CoreError::PngRenderFailed(e.to_string()))?;

    Ok(buf)
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qr::{QrEcLevel, QrOptions, generate_qr};

    #[test]
    fn test_qr_to_svg() {
        let opts = QrOptions::default();
        let matrix = generate_qr("https://example.com", &opts).unwrap();
        let svg = qr_to_svg(&matrix, &opts).unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("rect"));
    }

    #[test]
    fn test_qr_to_png() {
        let opts = QrOptions::default();
        let matrix = generate_qr("hello", &opts).unwrap();
        let png = qr_to_png(&matrix, &opts).unwrap();
        assert!(!png.is_empty());
        // PNG magic bytes
        assert_eq!(&png[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_qr_to_svg_custom_colors() {
        let opts = QrOptions {
            fg_color: [255, 0, 0, 255],
            bg_color: [0, 0, 255, 255],
            ..Default::default()
        };
        let matrix = generate_qr("test", &opts).unwrap();
        let svg = qr_to_svg(&matrix, &opts).unwrap();
        assert!(svg.contains("rgb(255,0,0)"));
        assert!(svg.contains("rgb(0,0,255)"));
    }

    #[test]
    fn test_barcode_svg_and_png() {
        use crate::barcode::{BarcodeType, generate_barcode};
        let matrix = generate_barcode(
            "12345678",
            BarcodeType::Code128,
            100,
            2,
            [0, 0, 0, 255],
            [255, 255, 255, 255],
        )
        .unwrap();

        let svg = barcode_to_svg(&matrix).unwrap();
        assert!(svg.starts_with("<svg"));

        let png = barcode_to_png(&matrix).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_qr_to_png_with_logo() {
        let opts = QrOptions {
            ec_level: QrEcLevel::H, // High EC for logo overlay
            ..Default::default()
        };
        let matrix = generate_qr("https://example.com", &opts).unwrap();

        // Create a tiny 32x32 red square as logo
        let mut logo_img: RgbaImage = ImageBuffer::new(32, 32);
        for pixel in logo_img.pixels_mut() {
            *pixel = Rgba([255, 0, 0, 255]);
        }
        let mut logo_buf = Vec::new();
        logo_img
            .write_to(
                &mut std::io::Cursor::new(&mut logo_buf),
                image::ImageFormat::Png,
            )
            .unwrap();

        let result = qr_to_png_with_logo(&matrix, &opts, &logo_buf, 0.25);
        assert!(result.is_ok());
        let png = result.unwrap();
        assert!(!png.is_empty());
    }
}
