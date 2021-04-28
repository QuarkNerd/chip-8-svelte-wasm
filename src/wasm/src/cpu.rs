use js_sys::Uint8Array;

pub struct CPU {
    keyboard: [u8; 0x10],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            keyboard: [0; 0x10]
        }
    }

    pub fn keys(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.keyboard) }
    }
}