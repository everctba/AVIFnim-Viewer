use std::io::Cursor;

use image::{ExtendedColorType, ImageEncoder, ImageReader};

/// Limite de pixels para decodificacao de PNM (evita OOM com headers maliciosos).
const MAX_PIXELS: u64 = 100_000_000; // 100 MPix
/// Limite de bytes do arquivo PNM.
const MAX_BYTES: u64 = 256 * 1024 * 1024;

pub fn is_pnm_ext(ext: &str) -> bool {
    matches!(ext, "ppm" | "pgm" | "pbm" | "pnm")
}

struct Header {
    width: u32,
    height: u32,
    _maxval: u32,
    _offset: usize,
}

fn skip_ws_and_comments(bytes: &[u8], mut i: usize) -> usize {
    loop {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        return i;
    }
}

fn parse_number(bytes: &[u8], mut i: usize) -> Result<(u32, usize), String> {
    i = skip_ws_and_comments(bytes, i);
    let start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if start == i {
        return Err("Cabecalho PNM invalido: numero esperado".into());
    }
    let value: u32 = std::str::from_utf8(&bytes[start..i])
        .map_err(|_| "Cabecalho PNM invalido".to_string())?
        .parse()
        .map_err(|_| "Cabecalho PNM invalido: numero fora de faixa".to_string())?;
    Ok((value, i))
}

fn parse_header(bytes: &[u8]) -> Result<Header, String> {
    if bytes.len() < 2 || bytes[0] != b'P' {
        return Err("Nao parece um arquivo PNM/PPM (falta o magic 'P')".into());
    }
    let kind = bytes[1];
    if !(b'1'..=b'6').contains(&kind) {
        return Err(format!("Formato PNM nao suportado: P{}", kind as char));
    }
    if bytes.len() as u64 > MAX_BYTES {
        return Err("Arquivo PNM muito grande (limite de 256 MB)".into());
    }
    let (width, i) = parse_number(bytes, 2)?;
    let (height, i) = parse_number(bytes, i)?;
    let (maxval, i) = match kind {
        b'1' | b'4' => (1u32, i),
        _ => {
            let (v, i) = parse_number(bytes, i)?;
            (v, i)
        }
    };
    if width == 0 || height == 0 {
        return Err("Dimensoes PNM invalidas (0 x 0)".into());
    }
    let pixels = width as u64 * height as u64;
    if pixels > MAX_PIXELS {
        return Err(format!(
            "Imagem PNM grande demais ({}x{} = {} MPix; limite 100 MPix)",
            width,
            height,
            pixels / 1_000_000
        ));
    }
    Ok(Header {
        width,
        height,
        _maxval: maxval,
        _offset: i,
    })
}

/// Decodifica PPM/PNM (P1-P6, 8/16-bit) e devolve PNG (8-bit RGB) + dimensoes.
pub fn decode_pnm_to_png(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32), String> {
    let header = parse_header(bytes)?;
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| format!("Falha ao reconhecer o PNM: {e}"))?;
    let image = reader
        .decode()
        .map_err(|e| format!("Falha ao decodificar PNM: {e}"))?;
    let (width, height) = (image.width(), image.height());
    if width != header.width || height != header.height {
        return Err("Cabecalho PNM nao confere com os dados da imagem".into());
    }
    let rgb = image.to_rgb8();
    let mut png: Vec<u8> = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png)
        .write_image(rgb.as_raw(), width, height, ExtendedColorType::Rgb8)
        .map_err(|e| format!("Falha ao gerar PNG do PNM: {e}"))?;
    Ok((png, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_magic_ok(png: &[u8]) -> bool {
        png.len() > 8 && png[..8] == [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
    }

    #[test]
    fn decodes_p6_binary() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"P6\n2 2\n255\n");
        bytes.extend_from_slice(&[0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]);
        let (png, w, h) = decode_pnm_to_png(&bytes).expect("P6 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn decodes_p3_ascii() {
        let bytes = b"P3\n2 2\n255\n0 0 0 255 0 0 0 255 0 0 0 255\n";
        let (png, w, h) = decode_pnm_to_png(bytes).expect("P3 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn rejects_unknown_magic() {
        let err = decode_pnm_to_png(b"P9\n2 2\n255\n").expect_err("P9 nao deve decodificar");
        assert!(err.contains("P9"), "erro inesperado: {err}");
    }

    #[test]
    fn rejects_huge_dimensions() {
        let err = decode_pnm_to_png(b"P6\n100000 100000\n255\n").expect_err("deve recusar");
        assert!(err.contains("grande demais"), "erro inesperado: {err}");
    }

    #[test]
    fn rejects_zero_dimensions() {
        let err = decode_pnm_to_png(b"P6\n0 10\n255\n").expect_err("deve recusar");
        assert!(err.contains("Dimensoes"), "erro inesperado: {err}");
    }

    #[test]
    fn recognizes_pnm_extensions() {
        assert!(is_pnm_ext("ppm"));
        assert!(is_pnm_ext("pnm"));
        assert!(!is_pnm_ext("avif"));
    }

    #[test]
    fn decodes_p1_bitmap_ascii() {
        let (png, w, h) = decode_pnm_to_png(b"P1\n2 2\n0 1\n1 0\n").expect("P1 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn decodes_p2_grayscale_ascii() {
        let (png, w, h) = decode_pnm_to_png(b"P2\n2 2\n255\n0 64 128 255\n").expect("P2 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn decodes_p4_bitmap_binary() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"P4\n2 2\n");
        // P4: cada linha e preenchida ate o proximo byte (2 bits usados por linha).
        bytes.extend_from_slice(&[0b1100_0000, 0b0000_0000]);
        let (png, w, h) = decode_pnm_to_png(&bytes).expect("P4 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn decodes_p5_grayscale_binary() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"P5\n2 2\n255\n");
        bytes.extend_from_slice(&[0, 64, 128, 255]);
        let (png, w, h) = decode_pnm_to_png(&bytes).expect("P5 deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn decodes_p6_16bit_as_8bit_png() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"P6\n2 2\n65535\n");
        for value in [0u16, 65535, 32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        let (png, w, h) = decode_pnm_to_png(&bytes).expect("P6 16-bit deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }
}

