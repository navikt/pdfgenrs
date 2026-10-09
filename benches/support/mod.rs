pub(crate) fn validate_pdf(bytes: &[u8]) -> anyhow::Result<()> {
    let end = bytes
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map_or(0, |index| index + 1);
    anyhow::ensure!(
        bytes.starts_with(b"%PDF-") && bytes[..end].ends_with(b"%%EOF"),
        "conversion did not produce a complete PDF"
    );
    Ok(())
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0_u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn chunk(png: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    png.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = png.len();
    png.extend_from_slice(kind);
    png.extend_from_slice(data);
    png.extend_from_slice(&crc32(&png[start..]).to_be_bytes());
}

pub(crate) fn raster_png(side: u32, variant: u8) -> Vec<u8> {
    assert!((1..=4096).contains(&side));
    let mut pixels = Vec::with_capacity(((side + 1) * side) as usize);
    for y in 0..side {
        pixels.push(0);
        for x in 0..side {
            pixels.push(((x * 13 + y * 29 + (x ^ y) * 7 + u32::from(variant)) % 256) as u8);
        }
    }
    let mut zlib = vec![0x78, 0x01];
    let blocks = pixels.chunks(65_535);
    let count = blocks.len();
    for (index, block) in blocks.enumerate() {
        zlib.push(u8::from(index + 1 == count));
        let length = block.len() as u16;
        zlib.extend_from_slice(&length.to_le_bytes());
        zlib.extend_from_slice(&(!length).to_le_bytes());
        zlib.extend_from_slice(block);
    }
    let (mut a, mut b) = (1_u32, 0_u32);
    for byte in &pixels {
        a = (a + u32::from(*byte)) % 65_521;
        b = (b + a) % 65_521;
    }
    zlib.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&side.to_be_bytes());
    ihdr.extend_from_slice(&side.to_be_bytes());
    ihdr.extend_from_slice(&[8, 0, 0, 0, 0]);
    chunk(&mut png, b"IHDR", &ihdr);
    chunk(&mut png, b"IDAT", &zlib);
    chunk(&mut png, b"IEND", &[]);
    png
}

pub(crate) fn unique_png(fixture: &[u8], sequence: u64) -> Vec<u8> {
    let mut png = Vec::with_capacity(fixture.len() + 48);
    png.extend_from_slice(&fixture[..fixture.len() - 12]);
    chunk(&mut png, b"tEXt", format!("request\0{sequence}").as_bytes());
    png.extend_from_slice(&fixture[fixture.len() - 12..]);
    png
}

pub(crate) fn check_fixtures() -> anyhow::Result<()> {
    anyhow::ensure!(crc32(b"123456789") == 0xcbf4_3926, "invalid PNG CRC");
    let png = raster_png(8, 0);
    anyhow::ensure!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "invalid PNG header");
    anyhow::ensure!(
        pdfgenrs::pdf::validate_image(&png, "/fixture.png", 4096, 4096 * 4096)? == (8, 8),
        "invalid raster dimensions"
    );
    anyhow::ensure!(
        unique_png(&png, 1) != unique_png(&png, 2),
        "request image variants must differ"
    );
    anyhow::ensure!(validate_pdf(b"not a PDF").is_err(), "invalid PDF accepted");
    validate_pdf(b"%PDF-1.7\n%%EOF")?;
    validate_pdf(b"%PDF-1.7\n%%EOF\r\n")?;
    Ok(())
}
