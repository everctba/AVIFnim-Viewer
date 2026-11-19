#![no_main]

use libfuzzer_sys::fuzz_target;

// Fuzzing do parser PNM/PPM: qualquer byte pode virar imagem maliciosa.
// O decoder pode retornar Err, mas nunca pode panicar, travar ou estourar memoria
// (os caps de 256 MB / 100 MPix ficam no proprio pnm.rs).
fuzz_target!(|data: &[u8]| {
    let _ = avifnim_viewer_lib::pnm::decode_pnm_to_png(data);
});
