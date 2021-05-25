use cpu::*;
use display::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_timer::Instant;
use web_sys::console;

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
            cpu: CPU::new(Display::new()),
        }
    }

    pub fn load_rom(&mut self, rom: Uint8Array) {
        self.reset();
        self.cpu.load_rom(rom);
    }

    pub fn on_animation_frame(&mut self) {
        self.cpu.cycle();
    }

    pub fn get_keys(&self) -> Uint8Array {
        self.cpu.keys()
    }

    pub fn get_display(&self) -> Uint8Array {
        self.cpu.display.pixels()
    }

    fn reset(&mut self) {
        self.cpu = CPU::new(Display::new());
    }
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

mod cpu;
mod display;
