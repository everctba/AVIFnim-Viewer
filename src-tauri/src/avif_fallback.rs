use base64::Engine as _;
use image::{ExtendedColorType, ImageEncoder};
use serde::Serialize;

/// Limites de seguranca do fallback AVIF (o webview pode nao suportar AVIF).
const MAX_PIXELS: u64 = 100_000_000; // 100 MPix por frame
const MAX_FRAMES: usize = 300;
const MAX_TOTAL_RGBA_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvifFramePayload {
    pub data_url: String,
    pub duration_ms: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvifPayload {
    pub width: u32,
    pub height: u32,
    pub animated: bool,
    pub frames: Vec<AvifFramePayload>,
}

fn msg(lang: &str, pt: String, en: String) -> String {
    if lang == "en" {
        en
    } else {
        pt
    }
}

fn encode_png(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut png = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png)
        .write_image(rgba, width, height, ExtendedColorType::Rgba8)
        .map_err(|e| format!("Falha ao gerar PNG do AVIF: {e}"))?;
    Ok(png)
}

fn png_data_url(rgba: &[u8], width: u32, height: u32) -> Result<String, String> {
    let png = encode_png(rgba, width, height)?;
    Ok(format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(png)
    ))
}

fn check_frame_size(width: u64, height: u64, lang: &str) -> Result<(), String> {
    let pixels = width * height;
    if pixels > MAX_PIXELS {
        return Err(msg(
            lang,
            format!(
                "AVIF grande demais ({width}x{height} = {} MPix; limite 100 MPix)",
                pixels / 1_000_000
            ),
            format!(
                "AVIF too large ({width}x{height} = {} MPix; 100 MPix limit)",
                pixels / 1_000_000
            ),
        ));
    }
    Ok(())
}

/// Decodifica AVIF (still ou animado) para PNGs RGBA em data URLs.
pub fn decode(data: &[u8], lang: &str) -> Result<AvifPayload, String> {
    let mut frames = Vec::new();
    let mut width = 0u32;
    let mut height = 0u32;
    let mut total_bytes = 0u64;
    let mut seq_error = None;

    match avif_rust::AvifSequenceDecoder::new(data) {
        Ok(mut decoder) => loop {
            match decoder.next_frame() {
                Ok(Some(frame)) => {
                    let image = frame.frame.to_rgba8().map_err(|e| {
                        msg(
                            lang,
                            format!("Falha ao converter frame AVIF: {e:?}"),
                            format!("Failed to convert AVIF frame: {e:?}"),
                        )
                    })?;
                    check_frame_size(image.width as u64, image.height as u64, lang)?;
                    if frames.is_empty() {
                        width = image.width as u32;
                        height = image.height as u32;
                    }
                    if frames.len() >= MAX_FRAMES {
                        return Err(msg(
                            lang,
                            format!("AVIF com frames demais (limite {MAX_FRAMES})"),
                            format!("AVIF has too many frames (limit {MAX_FRAMES})"),
                        ));
                    }
                    total_bytes += image.rgba.len() as u64;
                    if total_bytes > MAX_TOTAL_RGBA_BYTES {
                        return Err(msg(
                            lang,
                            "AVIF grande demais para decodificar".to_string(),
                            "AVIF too large to decode".to_string(),
                        ));
                    }
                    let duration_ms = frame.timing.duration_ms.max(1).min(u32::MAX as u64) as u32;
                    frames.push(AvifFramePayload {
                        data_url: png_data_url(
                            &image.rgba,
                            image.width as u32,
                            image.height as u32,
                        )?,
                        duration_ms,
                    });
                }
                Ok(None) => break,
                Err(e) => {
                    seq_error = Some(e);
                    break;
                }
            }
        },
        Err(e) => seq_error = Some(e),
    }

    if !frames.is_empty() {
        return Ok(AvifPayload {
            width,
            height,
            animated: frames.len() > 1,
            frames,
        });
    }

    // Still image (ou sequencia que nao pode ser lida como AVIS).
    match avif_rust::image_from_bytes(data) {
        Ok(image) => {
            check_frame_size(image.width as u64, image.height as u64, lang)?;
            let data_url = png_data_url(&image.rgba, image.width as u32, image.height as u32)?;
            Ok(AvifPayload {
                width: image.width as u32,
                height: image.height as u32,
                animated: false,
                frames: vec![AvifFramePayload {
                    data_url,
                    duration_ms: 0,
                }],
            })
        }
        Err(e) => {
            let detail = seq_error
                .map(|seq| format!("{e:?}; sequencia: {seq:?}"))
                .unwrap_or_else(|| format!("{e:?}"));
            Err(msg(
                lang,
                format!("Falha ao decodificar AVIF: {detail}"),
                format!("Failed to decode AVIF: {detail}"),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str) -> Vec<u8> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("tests")
            .join("assets")
            .join(name);
        std::fs::read(&path).unwrap_or_else(|e| panic!("ler {}: {e}", path.display()))
    }

    #[test]
    fn decodes_animated_avif_frames() {
        let payload = decode(&asset("anim.avif"), "pt").expect("anim.avif deve decodificar");
        assert_eq!((payload.width, payload.height), (96, 64));
        assert!(payload.animated);
        assert_eq!(payload.frames.len(), 8);
        assert_eq!(payload.frames[0].duration_ms, 100);
        assert!(payload.frames[0]
            .data_url
            .starts_with("data:image/png;base64,"));
    }

    #[test]
    fn decodes_still_avif() {
        let payload = decode(&asset("still.avif"), "pt").expect("still.avif deve decodificar");
        assert!(!payload.animated);
        assert_eq!(payload.frames.len(), 1);
        assert!(payload.frames[0]
            .data_url
            .starts_with("data:image/png;base64,"));
    }

    #[test]
    fn corrupt_avif_is_error() {
        let err = decode(b"nao e avif", "en").expect_err("deve falhar");
        assert!(err.contains("AVIF"), "erro inesperado: {err}");
    }
}
