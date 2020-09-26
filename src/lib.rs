#![recursion_limit = "1024"]

#[macro_use()]
extern crate serde;

mod agents;
mod api;
mod app;
mod components;
mod design_system;
mod pages;
mod router;
mod types;
mod utils;

use wasm_bindgen::prelude::*;

use image;
use rqrr;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}


#[wasm_bindgen]
pub fn decode_qr(bytes: &[u8]) -> String {
    let img = match image::load_from_memory(&bytes) {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed when trying to load image"),
    };

    let img = img.to_luma();

    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    if grids.len() != 1 {
        return format!("{}", "[Error] No QR code detected in image")
    }

    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed decoding the image"),
    };

    return format!("{}", content);
}