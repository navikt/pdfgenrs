/// Detects the image format from magic bytes and returns its canonical file extension.
///
/// Returns `None` if the format is unrecognised.
pub(super) fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if data.starts_with(&[0xFF, 0xD8]) {
        Some("jpg")
    } else if data.starts_with(b"RIFF") && data.len() >= 30 && &data[8..12] == b"WEBP" {
        Some("webp")
    } else if is_svg(data) {
        Some("svg")
    } else {
        None
    }
}

/// Extracts (width, height) from PNG, JPEG, WebP, or SVG image bytes by parsing headers.
///
/// Returns `None` if the format is unrecognised or the header is too short.
#[cfg(test)]
pub(super) fn image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let fmt = detect_image_format(data)?;
    image_dimensions_by_format(data, fmt)
}

/// Extracts (width, height) for a known image format without re-detecting the format.
///
/// `fmt` must be one of `"png"`, `"jpg"`, `"webp"`, or `"svg"` as returned by
/// [`detect_image_format`]. Returns `None` if the header is too short or malformed.
pub(super) fn image_dimensions_by_format(data: &[u8], fmt: &str) -> Option<(u32, u32)> {
    match fmt {
        "png" => png_dimensions(data),
        "jpg" => jpeg_dimensions(data),
        "webp" => webp_dimensions(data),
        "svg" => svg_dimensions(data),
        _ => None,
    }
}

fn png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 {
        return None;
    }
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((width, height))
}

fn jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let mut i = 2;
    while data.len() >= i + 2 {
        if data[i] != 0xFF {
            return None;
        }
        let marker = data[i + 1];
        if marker == 0xD9 {
            return None;
        }
        if matches!(marker, 0xC0..=0xC3 | 0xC9..=0xCB) {
            if data.len() < i + 9 {
                return None;
            }
            let height = u32::from(u16::from_be_bytes([data[i + 5], data[i + 6]]));
            let width = u32::from(u16::from_be_bytes([data[i + 7], data[i + 8]]));
            return Some((width, height));
        }
        if data.len() < i + 4 {
            return None;
        }
        let seg_len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
        i += 2 + seg_len;
    }
    None
}

fn webp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 30 {
        return None;
    }
    // VP8 lossy format
    if &data[12..16] == b"VP8 " {
        // VP8 bitstream header starts at offset 20 (after the chunk header)
        // Frame tag at bytes 23..26 contains width/height
        let width = u32::from(u16::from_le_bytes([data[26], data[27]])) & 0x3FFF;
        let height = u32::from(u16::from_le_bytes([data[28], data[29]])) & 0x3FFF;
        return Some((width, height));
    }
    // VP8L lossless format
    if &data[12..16] == b"VP8L" && data.len() >= 25 && data[20] == 0x2F {
        // Signature byte at offset 20 verified; width/height packed in next 4 bytes
        let b0 = u32::from(data[21]);
        let b1 = u32::from(data[22]);
        let b2 = u32::from(data[23]);
        let b3 = u32::from(data[24]);
        let width = (b0 | (b1 << 8)) & 0x3FFF;
        let height = ((b1 >> 6) | (b2 << 2) | (b3 << 10)) & 0x3FFF;
        return Some((width + 1, height + 1));
    }
    // VP8X extended format
    if &data[12..16] == b"VP8X" && data.len() >= 30 {
        let width = u32::from(data[24]) | (u32::from(data[25]) << 8) | (u32::from(data[26]) << 16);
        let height = u32::from(data[27]) | (u32::from(data[28]) << 8) | (u32::from(data[29]) << 16);
        return Some((width + 1, height + 1));
    }
    None
}

/// Checks whether bytes start with an SVG document (XML declaration or `<svg` tag).
fn is_svg(data: &[u8]) -> bool {
    let trimmed = trim_leading_whitespace(data);
    trimmed.starts_with(b"<?xml") || trimmed.starts_with(b"<svg")
}

/// Extracts width and height from an SVG root element.
///
/// Parses the `viewBox`, `width`, and `height` attributes. If `width`/`height`
/// are present as unitless numbers or pixel values they take priority; otherwise
/// falls back to the viewBox dimensions.
fn svg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let text = std::str::from_utf8(data).ok()?;
    let svg_start = text.find("<svg")?;
    let svg_tag_end = text[svg_start..].find('>')? + svg_start;
    let svg_tag = &text[svg_start..=svg_tag_end];

    let width_attr = extract_svg_attr(svg_tag, "width");
    let height_attr = extract_svg_attr(svg_tag, "height");

    // Try width/height attributes first (only unitless or px values)
    if let (Some(w), Some(h)) = (
        width_attr.and_then(parse_svg_length),
        height_attr.and_then(parse_svg_length),
    ) {
        return Some((w, h));
    }

    // Fall back to viewBox (handles both space-separated and comma-separated values)
    if let Some(vb) = extract_svg_attr(svg_tag, "viewBox") {
        let parts: Vec<&str> = vb
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() == 4 {
            let w: f64 = parts[2].parse().ok()?;
            let h: f64 = parts[3].parse().ok()?;
            if w > 0.0 && h > 0.0 {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                return Some((w as u32, h as u32));
            }
        }
    }

    None
}

/// Extracts the value of an attribute from an SVG/XML tag string.
fn extract_svg_attr<'a>(tag: &'a str, attr_name: &str) -> Option<&'a str> {
    // Match attr_name followed by = and a quoted value
    let search_len = attr_name.len() + 1; // attr_name + '='
    let mut start = 0;
    let pos = loop {
        let rel = tag[start..].find(attr_name)?;
        let abs = start + rel;
        // Verify word boundary: the character before the match must be whitespace or '<'
        let prev = tag[..abs].chars().next_back();
        if prev.is_none_or(|c| c.is_ascii_whitespace() || c == '<')
            && tag.as_bytes().get(abs + attr_name.len()) == Some(&b'=')
        {
            break abs;
        }
        start = abs + tag[abs..].chars().next()?.len_utf8();
    };
    let after_eq = &tag[pos + search_len..];
    let quote = after_eq.as_bytes().first()?;
    if *quote != b'"' && *quote != b'\'' {
        return None;
    }
    let value_start = 1;
    let value_end = after_eq[value_start..].find(*quote as char)? + value_start;
    Some(&after_eq[value_start..value_end])
}

/// Parses a unitless or pixel SVG length value to an u32.
fn parse_svg_length(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    let numeric = if let Some(stripped) = trimmed.strip_suffix("px") {
        stripped.trim()
    } else if trimmed.ends_with(|c: char| c.is_alphabetic() || c == '%') {
        // Other units (em, pt, cm, etc.) - can't reliably convert to pixels
        return None;
    } else {
        trimmed
    };
    let f: f64 = numeric.parse().ok()?;
    if f > 0.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some(f as u32)
    } else {
        None
    }
}

/// Trims leading ASCII whitespace bytes.
fn trim_leading_whitespace(data: &[u8]) -> &[u8] {
    let start = data
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(data.len());
    &data[start..]
}

#[cfg(test)]
mod tests {
    use super::super::testutil::root_dir;
    use super::*;
    use anyhow::Result;
    use std::fs;

    #[test]
    fn image_dimensions_png_parses_correctly() -> Result<()> {
        let data = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        let dims = image_dimensions(&data);
        assert_eq!(dims, Some((2201, 1386)));
        Ok(())
    }

    #[test]
    fn image_dimensions_returns_none_for_short_data() {
        assert_eq!(image_dimensions(&[0x89, 0x50, 0x4E, 0x47]), None);
        assert_eq!(image_dimensions(&[0xFF, 0xD8]), None);
        assert_eq!(image_dimensions(&[]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_unknown_format() {
        assert_eq!(image_dimensions(b"GIF89a"), None);
    }

    // --- Fuzz-style edge case tests for PNG parser ---

    #[test]
    fn png_dimensions_returns_none_for_valid_magic_but_truncated_ihdr() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn png_dimensions_handles_exactly_24_bytes() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&100u32.to_be_bytes());
        data.extend_from_slice(&200u32.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((100, 200)));
    }

    #[test]
    fn png_dimensions_zero_width_and_height() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((0, 0)));
    }

    #[test]
    fn png_dimensions_max_u32_values() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((u32::MAX, u32::MAX)));
    }

    #[test]
    fn png_dimensions_square_image_not_landscape() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&500u32.to_be_bytes());
        data.extend_from_slice(&500u32.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| w <= h),
            "Square image should not be detected as landscape"
        );
    }

    #[test]
    fn png_dimensions_portrait_image() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&100u32.to_be_bytes());
        data.extend_from_slice(&200u32.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| h > w),
            "Image should be portrait"
        );
    }

    // --- Fuzz-style edge case tests for JPEG parser ---

    #[test]
    fn jpeg_dimensions_returns_none_for_soi_only() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_immediate_eoi() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8, 0xFF, 0xD9]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_when_non_ff_byte_encountered() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8, 0x00, 0xC0]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_sof_marker_with_truncated_data() {
        let data = [0xFF, 0xD8, 0xFF, 0xC0, 0x00, 0x11, 0x08];
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn jpeg_dimensions_parses_sof0_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((640, 480)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof1_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC1];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&100u16.to_be_bytes());
        data.extend_from_slice(&200u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((200, 100)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof2_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC2];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&300u16.to_be_bytes());
        data.extend_from_slice(&400u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((400, 300)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof3_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC3];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&768u16.to_be_bytes());
        data.extend_from_slice(&1024u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((1024, 768)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof9_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC9];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((640, 480)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof10_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xCA];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&100u16.to_be_bytes());
        data.extend_from_slice(&200u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((200, 100)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof11_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xCB];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&300u16.to_be_bytes());
        data.extend_from_slice(&400u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((400, 300)));
    }

    #[test]
    fn jpeg_dimensions_skips_non_sof_segments_before_sof() {
        let mut data = vec![0xFF, 0xD8];
        data.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10]);
        data.extend_from_slice(&[0x00; 14]);
        data.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x11, 0x08]);
        data.extend_from_slice(&1080u16.to_be_bytes());
        data.extend_from_slice(&1920u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((1920, 1080)));
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_truncated_segment_length() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn jpeg_dimensions_zero_dimensions() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((0, 0)));
    }

    #[test]
    fn jpeg_dimensions_landscape_detection() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| w > h),
            "640x480 should be landscape"
        );
    }

    // --- Malformed image edge cases ---

    #[test]
    fn image_dimensions_returns_none_for_single_byte() {
        assert_eq!(image_dimensions(&[0xFF]), None);
        assert_eq!(image_dimensions(&[0x89]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_partial_png_magic() {
        assert_eq!(image_dimensions(b"\x89PNG\r\n"), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_all_zeros() {
        assert_eq!(image_dimensions(&[0u8; 100]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_all_0xff() {
        assert_eq!(image_dimensions(&[0xFF; 100]), None);
    }

    // --- WebP dimension tests ---

    #[test]
    fn webp_vp8_lossy_dimensions() {
        // Minimal VP8 lossy WebP: RIFF header + VP8 chunk
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&100u32.to_le_bytes()); // file size (not validated)
        data.extend_from_slice(b"WEBP");
        data.extend_from_slice(b"VP8 ");
        data.extend_from_slice(&18u32.to_le_bytes()); // chunk size
        // VP8 bitstream: 3-byte frame tag
        data.extend_from_slice(&[0x00, 0x00, 0x00]); // frame tag (offset 20-22)
        // 3-byte sync code
        data.extend_from_slice(&[0x9D, 0x01, 0x2A]); // sync code (offset 23-25)
        // width and height (little-endian 14 bits each)
        data.extend_from_slice(&320u16.to_le_bytes()); // width=320 (offset 26-27)
        data.extend_from_slice(&240u16.to_le_bytes()); // height=240 (offset 28-29)
        assert_eq!(image_dimensions(&data), Some((320, 240)));
    }

    #[test]
    fn webp_vp8x_extended_dimensions() {
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(b"WEBP");
        data.extend_from_slice(b"VP8X");
        data.extend_from_slice(&10u32.to_le_bytes()); // chunk size
        data.extend_from_slice(&[0u8; 4]); // flags (offset 20..24)
        // Canvas width - 1 (3 bytes LE): 799 = width 800
        let w_minus_1: u32 = 800 - 1;
        data.push((w_minus_1 & 0xFF) as u8);
        data.push(((w_minus_1 >> 8) & 0xFF) as u8);
        data.push(((w_minus_1 >> 16) & 0xFF) as u8);
        // Canvas height - 1 (3 bytes LE): 599 = height 600
        let h_minus_1: u32 = 600 - 1;
        data.push((h_minus_1 & 0xFF) as u8);
        data.push(((h_minus_1 >> 8) & 0xFF) as u8);
        data.push(((h_minus_1 >> 16) & 0xFF) as u8);
        assert_eq!(image_dimensions(&data), Some((800, 600)));
    }

    #[test]
    fn webp_returns_none_for_truncated_data() {
        let data = b"RIFF\x00\x00\x00\x00WEBP";
        assert_eq!(image_dimensions(data), None);
    }

    // --- SVG dimension tests ---

    #[test]
    fn svg_dimensions_from_width_height_attrs() {
        let svg = br#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((800, 600)));
    }

    #[test]
    fn svg_dimensions_from_viewbox() {
        let svg = br#"<svg viewBox="0 0 1024 768" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((1024, 768)));
    }

    #[test]
    fn svg_dimensions_with_px_suffix() {
        let svg = br#"<svg width="400px" height="300px" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((400, 300)));
    }

    #[test]
    fn svg_dimensions_with_xml_declaration() {
        let svg = br#"<?xml version="1.0" encoding="UTF-8"?><svg width="200" height="100"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((200, 100)));
    }

    #[test]
    fn svg_dimensions_with_leading_whitespace() {
        let svg = b"  \n  <svg width=\"50\" height=\"75\"></svg>";
        assert_eq!(image_dimensions(svg), Some((50, 75)));
    }

    #[test]
    fn svg_dimensions_returns_none_for_em_units() {
        let svg = br#"<svg width="10em" height="10em"></svg>"#;
        assert_eq!(image_dimensions(svg), None);
    }

    #[test]
    fn svg_dimensions_falls_back_to_viewbox_when_units_present() {
        let svg =
            br#"<svg width="10cm" height="5cm" viewBox="0 0 300 150" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((300, 150)));
    }

    #[test]
    fn svg_landscape_detected() {
        let svg = br#"<svg width="1920" height="1080"></svg>"#;
        assert!(
            image_dimensions(svg).is_some_and(|(w, h)| w > h),
            "Wide SVG should be landscape"
        );
    }

    // --- SVG dimension edge cases ---

    #[test]
    fn svg_dimensions_with_comma_separated_viewbox() {
        let svg = br#"<svg viewBox="0,0,640,480" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((640, 480)));
    }

    #[test]
    fn svg_dimensions_returns_none_for_empty_svg_tag() {
        let svg = br#"<svg></svg>"#;
        assert_eq!(image_dimensions(svg), None);
    }

    #[test]
    fn svg_dimensions_with_single_quotes() {
        let svg = b"<svg width='120' height='80'></svg>";
        assert_eq!(image_dimensions(svg), Some((120, 80)));
    }

    #[test]
    fn svg_dimensions_no_partial_attr_name_match() {
        // stroke-width= must not be matched when searching for width=
        let svg =
            br#"<svg stroke-width="5" viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((200, 100)));
    }

    #[test]
    fn svg_dimensions_skip_unicode_attribute_names() {
        for svg in [
            r#"<svg widthé="unused" width="10" height="10"></svg>"#,
            r#"<svg height😀="unused" width="10" height="10"></svg>"#,
            r#"<svg éwidth="unused" width="10" height="10"></svg>"#,
            r#"<svg viewBox界="unused" viewBox="0 0 10 10"></svg>"#,
        ] {
            assert_eq!(image_dimensions(svg.as_bytes()), Some((10, 10)));
        }
        let svg = r#"<svg widthé="unused" height="10"></svg>"#;
        assert_eq!(image_dimensions(svg.as_bytes()), None);
    }

    #[test]
    fn detect_image_format_returns_correct_format_for_each_type() -> Result<()> {
        let png = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        assert_eq!(detect_image_format(&png), Some("png"));

        let jpg = fs::read(root_dir().join("resources").join("NAVLogoRed.jpg"))?;
        assert_eq!(detect_image_format(&jpg), Some("jpg"));

        let webp = fs::read(root_dir().join("resources").join("test.webp"))?;
        assert_eq!(detect_image_format(&webp), Some("webp"));

        let svg = fs::read(root_dir().join("resources").join("pdfgenrs-logo.svg"))?;
        assert_eq!(detect_image_format(&svg), Some("svg"));

        assert_eq!(detect_image_format(b"not an image"), None);
        Ok(())
    }
}
