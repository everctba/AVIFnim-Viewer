//! Fallback AVIF para Linux usando libavif + dav1d (CPU, nativo).
//! Windows/macOS continuam usando `avif-rust`; este módulo so compila no Linux.
#![cfg(target_os = "linux")]

use base64::Engine as _;
use image::{ExtendedColorType, ImageEncoder};
use libavif_sys::*;
use serde::Serialize;
use std::ffi::CStr;
use std::slice;

const MAX_PIXELS: u64 = 100_000_000; // 100 MPix por frame
const MAX_FRAMES: usize = 300;
const MAX_TOTAL_PNG_BYTES: u64 = 384 * 1024 * 1024;

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

fn check_frame_size(width: u64, height: u64, lang: &str) -> Result<(), String> {
    let pixels = width * height;
    if pixels > MAX_PIXELS {
        return Err(msg(
            lang,
            format!("AVIF grande demais ({width}x{height} = {} MPix; limite 100 MPix)", pixels / 1_000_000),
            format!("AVIF too large ({width}x{height} = {} MPix; 100 MPix limit)", pixels / 1_000_000),
        ));
    }
    Ok(())
}

fn png_data_url(rgba: &[u8], width: u32, height: u32) -> Result<(String, u64), String> {
    let mut png = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png)
        .write_image(rgba, width, height, ExtendedColorType::Rgba8)
        .map_err(|e| format!("Falha ao gerar PNG do AVIF: {e}"))?;
    let size = png.len() as u64;
    Ok((
        format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(png)
        ),
        size,
    ))
}

struct DecoderGuard(*mut avifDecoder);

impl Drop for DecoderGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                avifDecoderDestroy(self.0);
            }
        }
    }
}

struct RgbGuard(avifRGBImage);

impl Drop for RgbGuard {
    fn drop(&mut self) {
        unsafe {
            avifRGBImageFreePixels(&mut self.0);
        }
    }
}

unsafe fn result_error(decoder: *mut avifDecoder, result: avifResult, lang: &str) -> String {
    let base = CStr::from_ptr(avifResultToString(result))
        .to_string_lossy()
        .into_owned();
    let detail = if decoder.is_null() {
        base
    } else {
        let diag = &(*decoder).diag;
        let bytes: Vec<u8> = diag
            .error
            .iter()
            .map(|c| *c as u8)
            .take_while(|b| *b != 0)
            .collect();
        let diag_text = String::from_utf8_lossy(&bytes);
        if diag_text.is_empty() {
            base
        } else {
            format!("{base}: {diag_text}")
        }
    };
    msg(
        lang,
        format!("Falha ao decodificar AVIF: {detail}"),
        format!("Failed to decode AVIF: {detail}"),
    )
}

/// Decodifica AVIF (still ou animado) via libavif/dav1d para PNGs RGBA.
pub fn decode(data: &[u8], lang: &str) -> Result<AvifPayload, String> {
    unsafe {
        let decoder = avifDecoderCreate();
        if decoder.is_null() {
            return Err(msg(
                lang,
                "Nao foi possivel criar o decoder AVIF".to_string(),
                "Could not create the AVIF decoder".to_string(),
            ));
        }
        let _decoder_guard = DecoderGuard(decoder);

        // dav1d e o decoder CPU mais rapido; escolha explicita.
        (*decoder).codecChoice = AVIF_CODEC_CHOICE_DAV1D;
        (*decoder).maxThreads = std::thread::available_parallelism()
            .map(|n| n.get().min(8) as i32)
            .unwrap_or(1);

        let result = avifDecoderSetIOMemory(decoder, data.as_ptr(), data.len());
        if result != AVIF_RESULT_OK {
            return Err(result_error(decoder, result, lang));
        }

        let result = avifDecoderParse(decoder);
        if result != AVIF_RESULT_OK {
            return Err(result_error(decoder, result, lang));
        }

        let image_count = (*decoder).imageCount;
        if image_count <= 0 {
            return Err(msg(
                lang,
                "AVIF sem imagens".to_string(),
                "AVIF has no images".to_string(),
            ));
        }
        if image_count as usize > MAX_FRAMES {
            return Err(msg(
                lang,
                format!("AVIF com frames demais (limite {MAX_FRAMES})"),
                format!("AVIF has too many frames (limit {MAX_FRAMES})"),
            ));
        }

        let animated = image_count > 1;
        let mut frames = Vec::with_capacity(image_count as usize);
        let mut width = 0u32;
        let mut height = 0u32;
        let mut total_png_bytes = 0u64;

        for index in 0..image_count {
            let result = avifDecoderNextImage(decoder);
            if result != AVIF_RESULT_OK {
                return Err(result_error(decoder, result, lang));
            }

            let image = (*decoder).image;
            if image.is_null() {
                return Err(msg(
                    lang,
                    "Frame AVIF sem imagem".to_string(),
                    "AVIF frame has no image".to_string(),
                ));
            }
            let frame_width = (*image).width;
            let frame_height = (*image).height;
            check_frame_size(frame_width as u64, frame_height as u64, lang)?;
            if index == 0 {
                width = frame_width;
                height = frame_height;
            }

            let mut rgb = avifRGBImage::default();
            avifRGBImageSetDefaults(&mut rgb, image);
            rgb.depth = 8;
            rgb.format = AVIF_RGB_FORMAT_RGBA;
            rgb.ignoreAlpha = AVIF_FALSE;
            rgb.alphaPremultiplied = AVIF_FALSE;

            let result = avifRGBImageAllocatePixels(&mut rgb);
            if result != AVIF_RESULT_OK {
                return Err(result_error(decoder, result, lang));
            }
            let _rgb_guard = RgbGuard(rgb);

            let result = avifImageYUVToRGB(image, &mut rgb);
            if result != AVIF_RESULT_OK {
                return Err(result_error(decoder, result, lang));
            }

            let row_bytes = rgb.rowBytes as usize;
            let row_pixels = (frame_width as usize) * 4;
            let mut rgba = Vec::with_capacity(row_pixels * frame_height as usize);
            for row in 0..frame_height as usize {
                let src = rgb.pixels.add(row * row_bytes);
                rgba.extend_from_slice(slice::from_raw_parts(src, row_pixels));
            }

            let (data_url, png_size) = png_data_url(&rgba, frame_width, frame_height)?;
            total_png_bytes += png_size;
            if total_png_bytes > MAX_TOTAL_PNG_BYTES {
                return Err(msg(
                    lang,
                    format!(
                        "AVIF animado grande demais ({} frames, {}x{}; {:.0} MB de PNG; limite {} MB)",
                        frames.len() + 1,
                        frame_width,
                        frame_height,
                        total_png_bytes as f64 / (1024.0 * 1024.0),
                        MAX_TOTAL_PNG_BYTES / (1024 * 1024)
                    ),
                    format!(
                        "Animated AVIF too large ({} frames, {}x{}; {:.0} MB of PNG; {} MB limit)",
                        frames.len() + 1,
                        frame_width,
                        frame_height,
                        total_png_bytes as f64 / (1024.0 * 1024.0),
                        MAX_TOTAL_PNG_BYTES / (1024 * 1024)
                    ),
                ));
            }

            let duration_ms = if animated {
                let seconds = (*decoder).imageTiming.duration;
                if seconds > 0.0 {
                    (seconds * 1000.0).round().clamp(1.0, u32::MAX as f64) as u32
                } else {
                    100
                }
            } else {
                0
            };

            frames.push(AvifFramePayload {
                data_url,
                duration_ms,
            });
        }

        Ok(AvifPayload {
            width,
            height,
            animated,
            frames,
        })
    }
}
