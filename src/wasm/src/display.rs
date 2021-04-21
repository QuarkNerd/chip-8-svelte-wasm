use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

const COLS: usize = 64;
const ROWS: usize = 32;

#[wasm_bindgen]
pub struct Display {
    pixels: [u8; COLS*ROWS],
}

impl Display {
    pub fn new() -> Display {
        let mut disp = Display {
            pixels: [0; COLS*ROWS]
        };

        for n in 0..COLS*ROWS {
            disp.pixels[n] = rand::random::<u8>()%2;
        };

        disp
    }

    pub fn cells(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.pixels) }
    }
}
