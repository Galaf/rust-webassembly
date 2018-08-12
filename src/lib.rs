#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod mandelbrot;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub  fn run() {
    log("fill start");

    let _canvas = mandelbrot::draw(600, 600);

    // let byte_size = max_width * max_height * 4;
    // let s1 = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    // let canvas = mandelbrot::draw(max_width, max_height);

    // for i in 0..byte_size {
    //     let x = i / 4 % max_width;
    //     let y = 1 / 4 / max_width;

    //     s1[i] = canvas[x][y] as u8;
    // }
}
