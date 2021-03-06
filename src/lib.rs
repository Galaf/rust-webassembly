#![feature(use_extern_macros)]
#[macro_use]

extern crate serde_derive;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod mandelbrot;
use mandelbrot::get_mandelbrot_set;

#[derive(Deserialize)]
pub struct Configuration {
    pub iterations: usize,
    pub width: u32,
    pub height: u32,
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor)]
    pub fn new(arr: &Uint8ClampedArray, width: u32, height: u32) -> ImageData;
}

#[wasm_bindgen]
extern "C" {
    pub type Uint8ClampedArray;

    #[wasm_bindgen(constructor)]
    pub fn new(arr: &[u8]) -> Uint8ClampedArray;
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, js_name=putImageData)]
    pub fn put_image_data(this: &CanvasRenderingContext2D, image_data: &ImageData, p_1: i32, p_2: i32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub  fn draw(context: &CanvasRenderingContext2D, conf_js: &JsValue) {
    let configuration: Configuration = conf_js.into_serde().unwrap();

    log("Starting draw");
    log(&format!("Width = {}; Height = {}", configuration.width, configuration.height));

    let data = get_mandelbrot_set(&configuration);
    log(&format!("Generated {} numbers", data.len()));

    let uint8_array = Uint8ClampedArray::new(&data);

    log("uint8 array generated");

    context.put_image_data(&ImageData::new(&uint8_array, configuration.width, configuration.height), 0, 0);
}
