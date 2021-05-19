use crate::*;
use js_sys::Uint8Array;
use web_sys::console;

static SPEED: u8 = 9;

pub struct CPU {
    pub keyboard: [u8; 0x10],
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
        let mut cpu =
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
        };
        cpu.load_sprites_into_memory();
        cpu
    }

    pub fn keys(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.keyboard) }
    }

    pub fn load_rom(&mut self, rom: Uint8Array) {
        for loc in 0..rom.length() {
            self.memory[0x200 + loc as usize] = rom.get_index(loc);
        }
    }

    pub fn cycle(&mut self) {
        for _ in 0..SPEED {
            self.execute_next_instruction();
        }

        //timers
        if self.delay > 0 {
            self.delay -= 1;
        }
    }

    fn execute_next_instruction(&mut self) {
        // Instruction has 4 Base-16 characters
        // 0x0nnn (addr) or 0x0xyn (n = nibble) or 0x00kk (byte) 0xC000 (code)
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[self.pc as usize + 1] as u16;
        let code = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nibble = opcode & 0x000F;

        self.pc += 2;

        match (code, x, y, nibble) {
            (0, 0, 0xE, 0) => self.display.clear(),
            (0, 0, 0xE, 0xE) => {
                self.stack_pointer -= 1;
                self.pc = self.stack[self.stack_pointer]
            },
            (1, _, _, _) => self.pc = CPU::get_nnn(opcode),
            (2, _, _, _) => {
                self.stack[self.stack_pointer] = self.pc;
                self.stack_pointer += 1;
                self.pc = CPU::get_nnn(opcode);
            },
            (3, x, _, _) => if self.v[x] == CPU::get_kk(opcode) { self.pc += 2 },
            (4, x, _, _) => if self.v[x] != CPU::get_kk(opcode) { self.pc += 2 },
            (5, x, y, 0) => if self.v[x] == self.v[y] { self.pc += 2 },
            (6, x, _, _) => self.v[x] = CPU::get_kk(opcode),
            (7, x, _, _) => self.v[x] = self.v[x].wrapping_add(CPU::get_kk(opcode)),
            (8, x, y, 0) => self.v[x] = self.v[y],
            (8, x, y, 1) => self.v[x] |= self.v[y],
            (8, x, y, 2) => self.v[x] &= self.v[y],
            (8, x, y, 3) => self.v[x] ^= self.v[y],
            (8, x, y, 4) => {
                let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = carry as u8;
                self.v[x] = sum;
            },
            (8, x, y, 5) => {
                let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = !borrow as u8;
                self.v[x] = diff;
            },
            (8, x, _, 6) => {
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            },
            (8, x, y, 7) => {
                let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = !borrow as u8;
                self.v[x] = diff;
            },
            (8, x, _, 0xE) => {
                self.v[0xF] = (self.v[x] & 0x80); //EXPLAIN
                self.v[x] <<= 1;
            },
            (9, x, y, 0) => if self.v[x] != self.v[y] { self.pc += 2 },
            (0xA, _, _, _) => self.i = CPU::get_nnn(opcode),
            (0xB, _, _, _) => self.pc = self.v[0] as u16 + CPU::get_nnn(opcode),
            (0xC, x, _, _) => self.v[x] = rand::random::<u8>() & CPU::get_kk(opcode),
            (0xD, x, y, nibble) => {
                self.v[0xF] = 0;
                for y_diff in 0..nibble {
                    let y = self.v[y] + y_diff as u8; // potential overflow
                    let mut sprite_byte = self.memory[(self.i + y_diff) as usize];
                    for x_diff in 0..8 {
                        let x = self.v[x] + x_diff as u8; // potential overflow

                        if (sprite_byte & 0x80) >> 7 == 1 {
                            if self.display.set_pixel(x, y) {
                                self.v[0xF] = 1;
                            }
                        }

                        sprite_byte <<= 1;
                    }
                }
            },
            (0xE, x, 9, 0xE) => if self.keyboard[self.v[x] as usize] != 0 { self.pc += 2 },
            (0xE, x, 0xA, 1) => if self.keyboard[self.v[x] as usize] == 0 { self.pc += 2 },
            (0xF, x, 0, 7) => self.v[x] = self.delay,
            (0xF, x, 0, 0xA) => {
                self.pc -= 2;

                for (key, pressed) in self.keyboard.iter().enumerate() {
                    if *pressed != 0 {
                        self.pc += 2;
                        self.v[x] = key as u8;
                        break;
                    }
                }
            },
            (0xF, x, 1, 5) => self.delay = self.v[x],
            (0xF, x, 1, 8) => self.sound = self.v[x],
            (0xF, x, 1, 0xE) => self.i += self.v[x] as u16, //potential overflow
            (0xF, x, 2, 9) => self.i = self.v[x] as u16 * 5,
            (0xF, x, 3, 3) => {
                let vx = self.v[x];
                let loc = self.i as usize;
                self.memory[loc] = vx / 100;
                self.memory[loc + 1] = (vx / 10) % 10;
                self.memory[loc + 2] = vx % 10;
            },
            (0xF, x, 5, 5) => {
                let loc = self.i as usize;
                for register in 0..(x +1) {
                    self.memory[loc + register] = self.v[register];
                }
            },
            (0xF, x, 6, 5) => {
                let loc = self.i as usize;
                for register in 0..(x +1) {
                    self.v[register] = self.memory[loc + register];
                }
            },
            _ => {
                console::log_1(&format!("ERROR: Invalid opcode {}", opcode).into());
                panic!("Invalid opcode")
            },
        }
    }

    fn get_nnn(opcode: u16) -> u16 {
        opcode & 0x0FFF
    }

    fn get_kk(opcode: u16) -> u8 {
        opcode as u8 & 0x00FF
    }

    fn load_sprites_into_memory(&mut self) {
        let sprites: [u8; 0x50] = [
            0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
            0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
            0x90, 0x90, 0xf0, 0x10, 0x10, // 4
            0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
            0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
            0xf0, 0x10, 0x20, 0x40, 0x40, // 7
            0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
            0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
            0xf0, 0x90, 0xf0, 0x90, 0x90, // A
            0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
            0xf0, 0x80, 0x80, 0x80, 0xf0, // C
            0xe0, 0x90, 0x90, 0x90, 0xe0, // D
            0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
            0xf0, 0x80, 0xf0, 0x80, 0x80, // F
        ];

        for i in 0..0x50 {
            self.memory[i] = sprites[i];
        };
    }
}
