extern crate console_error_panic_hook;
use cpu::*;
use display::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use std::panic;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    cpu: CPU,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        set_panic_hook();
        Emulator {
            cpu: CPU::new(Display::new(),9),
        }
    }

    pub fn load_rom(&mut self, rom: Uint8Array, y_wrap: bool) {
        self.reset();
        self.cpu.display.y_wrap = y_wrap;
        self.cpu.load_rom(rom);
    }

    pub fn set_speed(&mut self, speed: js_sys::Number) {
        self.cpu.cycle_speed = speed.value_of() as u8
    }

    pub fn on_animation_frame(&mut self) {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        self.cpu.cycle();
    }

    pub fn get_keys(&self) -> Uint8Array {
        self.cpu.keys()
    }

    pub fn get_display(&self) -> Uint8Array {
        self.cpu.display.pixels()
    }

    pub fn get_sound(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.cpu.sound) }
    }

    fn reset(&mut self) {
        let speed: u8 = self.cpu.cycle_speed;
        self.cpu = CPU::new(Display::new(), speed);
    }
}

pub fn set_panic_hook() {
    // Call the `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

mod cpu;
mod display;
