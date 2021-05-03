use crate::*;
use js_sys::Uint8Array;

static SPEED: u8 = 9;

pub struct CPU {
    keyboard: [u8; 0x10],
    pub display: Display,
    memory: [u8; 4096],
    v: [u8; 0x10],
    delay: u8,
    sound: u8,
    pc: u16,
    stack_pointer: usize,
    stack: [u16; 0x10],
    i: u16,
}

impl CPU {
    pub fn new(display: Display) -> CPU {
        CPU {
            keyboard: [0; 0x10],
            display,
            memory: [0; 4096],
            v: [0; 0x10],
            delay: 0,
            sound: 0,
            pc: 0x200,
            stack_pointer: 0,
            stack: [0; 0x10],
            i: 0,
        }
    }

    pub fn keys(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.keyboard) }
    }
}
