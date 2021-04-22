use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use display::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    display: Display
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new()
        }
    }

    pub fn get_display(&self) -> Uint8Array {
        self.display.pixels()
    }

    pub fn flip(&mut self, x: &JsValue, y: &JsValue) -> bool {
        let x_n = x.as_f64().unwrap() as u8; 
        let y_n = y.as_f64().unwrap() as u8; 
        self.display.set_pixel(x_n, y_n)
    }
}

mod display;