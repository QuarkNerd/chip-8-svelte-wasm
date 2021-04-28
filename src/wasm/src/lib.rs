use wasm_timer::Instant;
use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Uint8Array;
use display::*;
use chip::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    chip: Chip,
    display: Display
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        set_panic_hook();
        Emulator {
            display: Display::new(),
            chip: Chip::new(),
        }
    }

    pub fn on_animation_frame(&mut self) {}

    pub fn get_keys(&self) -> Uint8Array {
        self.chip.keys()
    }

    pub fn get_display(&self) -> Uint8Array {
        self.display.pixels()
    }
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

mod display;
mod chip;
