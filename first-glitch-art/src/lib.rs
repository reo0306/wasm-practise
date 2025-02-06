#[allow(warnings)]
mod bindings;

use rand::Rng;
use bindings::exports::reo0306::glitch_art::png_glitchable::{Guest, ScanLine};

struct Component;

impl Guest for Component {
    fn glitch(mut scan_line: ScanLine) -> ScanLine {
        let mut rng = rand::thread_rng();
        scan_line.pixel_data[0] = rng.gen_range(0..=255);
        scan_line
    }
}

bindings::export!(Component with_types_in bindings);
