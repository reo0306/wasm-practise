use std::io::{Read, Write};
use png_glitch::{FilterType, PngGlitch};
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
};

#[allow(warnings)]
mod bindings;

use bindings::reo0306::glitch_art::png_glitchable::FilterType as WasmFilterType;
use bindings::reo0306::glitch_art::png_glitchable::ScanLine as WasmScanLine;
use bindings::reo0306::glitch_art::png_glitchable::glitch;

impl From<WasmFilterType> for FilterType {
    fn from(value: WasmFilterType) -> Self {
        match value {
            WasmFilterType::None => FilterType::None,
            WasmFilterType::Up => FilterType::Up,
            WasmFilterType::Sub => FilterType::Sub,
            WasmFilterType::Average => FilterType::Average,
            WasmFilterType::Paeth => FilterType::Paeth,
        }
    }
}

impl From<FilterType> for WasmFilterType {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::None => WasmFilterType::None,
            FilterType::Up => WasmFilterType::Up,
            FilterType::Sub => WasmFilterType::Sub,
            FilterType::Average => WasmFilterType::Average,
            FilterType::Paeth => WasmFilterType::Paeth,
        }
    }
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut buffer = vec![];
    req.body().read_to_end(&mut buffer)?;
    let mut png_glitch = PngGlitch::new(buffer)?;

    png_glitch.foreach_scanline(|scanline| {
        let mut pixel_data = vec![];
        if let Ok(_) = scanline.read_to_end(&mut pixel_data) {
            let filter_type = scanline.filter_type().into();
            let wasm_scan_line = WasmScanLine{filter_type, pixel_data};

            let returned_scan_line = glitch(&wasm_scan_line);

            scanline.set_filter_type(returned_scan_line.filter_type.into());
            let _ = scanline.write(&returned_scan_line.pixel_data);
        }
    });

    let mut buffer = vec![];
    png_glitch.encode(&mut buffer)?;

    let response = Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(buffer)
        .build();

    Ok(response)
}
