//! macOS: define o AVIFnim Viewer como handler padrao (LaunchServices) para os
//! formatos que ele visualiza. Chamado pelo botao das Configuracoes.
#![cfg(target_os = "macos")]

use core_foundation_sys::base::{CFRelease, CFTypeRef};
use core_foundation_sys::string::{kCFStringEncodingUTF8, CFStringCreateWithCString, CFStringRef};
use std::ffi::CString;
use std::ptr;

/// kLSRolesViewer
const LS_ROLES_VIEWER: u32 = 0x1;

/// Tipos que o botao "Definir como padrao" assume (nao mexe em PNG/JPEG).
const MIME_TYPES: &[&str] = &[
    "image/avif",
    "image/webp",
    "image/gif",
    "image/x-portable-pixmap",
    "image/x-portable-graymap",
    "image/x-portable-bitmap",
    "image/x-portable-anymap",
];

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSSetDefaultRoleHandlerForContentType(
        content_type: CFStringRef,
        role: u32,
        handler: CFStringRef,
    ) -> i32;
    fn UTTypeCreatePreferredIdentifierForTag(
        tag_class: CFStringRef,
        tag: CFStringRef,
        conforming_to: CFStringRef,
    ) -> CFStringRef;
}

unsafe fn cfstring(value: &str) -> Option<CFStringRef> {
    let c = CString::new(value).ok()?;
    let s = CFStringCreateWithCString(ptr::null(), c.as_ptr(), kCFStringEncodingUTF8);
    if s.is_null() {
        None
    } else {
        Some(s)
    }
}

pub fn set_default_handlers(bundle_id: &str) -> Result<(), String> {
    unsafe {
        let handler = cfstring(bundle_id)
            .ok_or_else(|| "Nao foi possivel criar o identificador do bundle".to_string())?;
        let tag_class = cfstring("public.mime-type")
            .ok_or_else(|| "Nao foi possivel criar a classe MIME".to_string())?;

        let mut failures = Vec::new();
        for mime in MIME_TYPES {
            let Some(tag) = cfstring(mime) else {
                failures.push(format!("{mime}: tag invalida"));
                continue;
            };
            let uti = UTTypeCreatePreferredIdentifierForTag(tag_class, tag, ptr::null());
            if uti.is_null() {
                failures.push(format!("{mime}: UTI nao encontrada"));
                CFRelease(tag as CFTypeRef);
                continue;
            }
            let status = LSSetDefaultRoleHandlerForContentType(uti, LS_ROLES_VIEWER, handler);
            if status != 0 {
                failures.push(format!("{mime}: erro {status}"));
            }
            CFRelease(uti as CFTypeRef);
            CFRelease(tag as CFTypeRef);
        }
        CFRelease(tag_class as CFTypeRef);
        CFRelease(handler as CFTypeRef);

        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Nao foi possivel definir todos os padroes: {}",
                failures.join("; ")
            ))
        }
    }
}
