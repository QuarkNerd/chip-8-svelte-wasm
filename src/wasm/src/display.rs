use js_sys::Uint8Array;
use web_sys::console;

const COLS: usize = 64;
const COLS_U8: u8 = 64;
const ROWS: usize = 32;
const ROWS_U8: u8 = 32;

pub struct Display {
    pixels: [u8; COLS * ROWS],
    pub y_wrap: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            pixels: [0; COLS * ROWS],
            y_wrap: false,
        }
    }

    pub fn set_pixel(&mut self, mut x: u8, mut y: u8) -> bool {
        x = x % COLS_U8;
        if y >= ROWS_U8 {
            if !self.y_wrap {
                return false;
            };
            y = y % ROWS_U8;
        };

        let pixel_num = x as usize + y as usize * COLS;
        self.pixels[pixel_num] ^= 1;
        self.pixels[pixel_num] == 0
    }

    pub fn clear(&mut self) {
        self.pixels = [0; COLS * ROWS];
    }

    pub fn pixels(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.pixels) }
    }
}
