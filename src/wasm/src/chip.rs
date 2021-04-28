use js_sys::Uint8Array;

pub struct Chip {
    keyboard: [u8; 0x10]
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            keyboard: [0; 0x10]
        }
    }

    pub fn keys(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.keyboard) }
    }
}