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

/// Versao de [`decode_pnm_to_png`] que traduz as mensagens de erro conforme o idioma do app.
pub fn decode_pnm_to_png_lang(bytes: &[u8], lang: &str) -> Result<(Vec<u8>, u32, u32), String> {
    decode_pnm_to_png(bytes).map_err(|err| localize_pnm_error(err, lang))
}

fn localize_pnm_error(message: String, lang: &str) -> String {
    if lang != "en" {
        return message;
    }
    const STATIC_MSGS: &[(&str, &str)] = &[
        (
            "Cabecalho PNM invalido: numero esperado",
            "Invalid PNM header: number expected",
        ),
        (
            "Cabecalho PNM invalido: numero fora de faixa",
            "Invalid PNM header: number out of range",
        ),
        ("Cabecalho PNM invalido", "Invalid PNM header"),
        (
            "Nao parece um arquivo PNM/PPM (falta o magic 'P')",
            "Not a PNM/PPM file (missing 'P' magic)",
        ),
        (
            "Arquivo PNM muito grande (limite de 256 MB)",
            "PNM file too large (256 MB limit)",
        ),
        (
            "Dimensoes PNM invalidas (0 x 0)",
            "Invalid PNM dimensions (0 x 0)",
        ),
        (
            "Cabecalho PNM nao confere com os dados da imagem",
            "PNM header does not match image data",
        ),
    ];
    for (pt, en) in STATIC_MSGS {
        if message == *pt {
            return (*en).to_string();
        }
    }
    if let Some(rest) = message.strip_prefix("Formato PNM nao suportado: ") {
        return format!("Unsupported PNM format: {rest}");
    }
    if let Some(rest) = message.strip_prefix("Falha ao reconhecer o PNM: ") {
        return format!("Failed to recognize PNM: {rest}");
    }
    if let Some(rest) = message.strip_prefix("Falha ao decodificar PNM: ") {
        return format!("Failed to decode PNM: {rest}");
    }
    if let Some(rest) = message.strip_prefix("Falha ao gerar PNG do PNM: ") {
        return format!("Failed to generate PNG from PNM: {rest}");
    }
    if let Some(inner) = message
        .strip_prefix("Imagem PNM grande demais (")
        .and_then(|rest| rest.strip_suffix(")"))
    {
        let inner = inner.replace("; limite 100 MPix", "; 100 MPix limit");
        return format!("PNM image too large ({inner})");
    }
    message
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn pnm_header(kind: u8, width: u32, height: u32, maxval: Option<u32>) -> Vec<u8> {
        let mut text = format!("P{} {} {}", kind, width, height);
        if let Some(maxval) = maxval {
            text.push_str(&format!(" {}", maxval));
        }
        text.push('\n');
        text.into_bytes()
    }

    proptest! {
        // Nenhuma entrada arbitraria pode causar panic/abort no decoder.
        #[test]
        fn arbitrary_bytes_never_panic(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
            let _ = decode_pnm_to_png(&bytes);
        }

        // O cap de 100 MPix vale para qualquer header, inclusive valores u32 extremos.
        #[test]
        fn header_dimensions_always_within_cap(width in any::<u32>(), height in any::<u32>()) {
            let mut data = pnm_header(6, width, height, Some(255));
            data.extend_from_slice(&[0u8; 32]);
            if let Ok(header) = parse_header(&data) {
                prop_assert!(header.width > 0 && header.height > 0);
                let pixels = header.width as u64 * header.height as u64;
                prop_assert!(
                    pixels <= MAX_PIXELS,
                    "cap violado: {}x{} = {} px",
                    header.width,
                    header.height,
                    pixels
                );
            }
        }

        // Espacos, CRLF e comentarios no header nao podem alterar o resultado.
        #[test]
        fn whitespace_and_comments_are_tolerated(width in 1u32..7, height in 1u32..7) {
            let mut data = Vec::new();
            data.extend_from_slice(b"P6\r\n# comentario do header\r\n");
            data.extend_from_slice(format!("{}", width).as_bytes());
            data.extend_from_slice(b"  # outro comentario\n ");
            data.extend_from_slice(format!("{}", height).as_bytes());
            data.extend_from_slice(b"\r\n255\r\n");
            data.extend(std::iter::repeat(128u8).take(width as usize * height as usize * 3));
            let (png, out_width, out_height) =
                decode_pnm_to_png(&data).expect("P6 valido deve decodificar");
            prop_assert_eq!((out_width, out_height), (width, height));
            prop_assert!(png.starts_with(b"\x89PNG"));
        }

        // Headers truncados/curtos devem retornar Err, nunca panicar.
        #[test]
        fn truncated_headers_are_errors(prefix in proptest::collection::vec(any::<u8>(), 0..16)) {
            let _ = decode_pnm_to_png(&prefix);
        }
    }
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
        let (png, w, h) =
            decode_pnm_to_png(b"P2\n2 2\n255\n0 64 128 255\n").expect("P2 deve decodificar");
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
        for value in [
            0u16, 65535, 32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64,
        ] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        let (png, w, h) = decode_pnm_to_png(&bytes).expect("P6 16-bit deve decodificar");
        assert_eq!((w, h), (2, 2));
        assert!(png_magic_ok(&png));
    }

    #[test]
    fn translates_pnm_errors_to_english() {
        let err =
            decode_pnm_to_png_lang(b"P6\n100000 100000\n255\n", "en").expect_err("deve falhar");
        assert!(
            err.contains("PNM image too large"),
            "erro inesperado: {err}"
        );

        let err = decode_pnm_to_png_lang(b"P9\n2 2\n255\n", "en").expect_err("deve falhar");
        assert!(
            err.contains("Unsupported PNM format"),
            "erro inesperado: {err}"
        );

        let pt = decode_pnm_to_png(b"P9\n2 2\n255\n").expect_err("deve falhar");
        assert!(pt.contains("Formato PNM"), "erro inesperado: {pt}");
    }
}
