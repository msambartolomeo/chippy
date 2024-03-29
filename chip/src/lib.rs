mod display;
mod hardware;
mod memory;
mod stack;

use display::Display;
use hardware::{Keyboard, Timer};
use memory::Memory;
use rand::Rng;
use stack::Stack;

use rand::thread_rng;

pub use display::{Screen, DISPLAY_HEIGHT, DISPLAY_WIDTH};
pub use hardware::Key;

const REGISTERS_COUNT: usize = 16;
const DEFAULT_ROM_START: u16 = 0x200;

pub struct Chip {
    v_registers: [u8; REGISTERS_COUNT],
    memory: Memory,
    delay_timer: Timer,
    sound_timer: Timer,
    stack: Stack,
    keyboard: Keyboard,
    display: Display,
}

pub struct Actions {
    pub draw: bool,
    pub beep: bool,
}

impl Default for Chip {
    fn default() -> Self {
        Self::new(DEFAULT_ROM_START)
    }
}

impl Chip {
    #[must_use]
    pub fn new(rom_start: u16) -> Self {
        Self {
            v_registers: [0; REGISTERS_COUNT],
            memory: Memory::new(rom_start),
            delay_timer: Timer::default(),
            sound_timer: Timer::default(),
            stack: Stack::default(),
            keyboard: Keyboard::default(),
            display: Display::default(),
        }
    }

    pub fn run_cycle(&mut self) -> Actions {
        self.process_instruction();

        self.delay_timer.countdown();
        let beep = self.sound_timer.countdown();
        let draw = self.display.must_draw();

        Actions { draw, beep }
    }

    pub fn load_rom(&mut self, path: String) -> Result<(), std::io::Error> {
        let rom = std::fs::read(path)?;

        self.memory.load_rom(&rom);

        Ok(())
    }

    #[must_use]
    pub const fn screen(&self) -> &Screen {
        self.display.screen()
    }

    pub fn press_key(&mut self, key: Key) {
        self.keyboard.press_key(key);
    }

    pub fn unpress_key(&mut self, key: Key) {
        self.keyboard.unpress_key(key);
    }

    #[inline]
    fn set_flag(&mut self, condition: bool) {
        self.v_registers[0xF] = u8::from(condition);
    }

    #[allow(clippy::too_many_lines)]
    fn process_instruction(&mut self) {
        let instruction = self.memory.get_current_instruction();

        let nibbles = instruction.get_nibbles();

        let v_x = self.v_registers[instruction.x];
        let v_y = self.v_registers[instruction.y];

        let mut jump = false;

        match nibbles {
            // 00E0 - CLS
            (0x0, 0x0, 0xE, 0x0) => self.display.clear(),
            // 00EE - RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.memory.pc_register = self.stack.pop();
            }
            // 0nnn - SYS addr - Ignored
            (0x0, _, _, _) => (),
            // 1nnn - JP addr
            (0x1, _, _, _) => {
                self.memory.pc_register = instruction.nnn;
                jump = true;
            }
            // 2nnn - CALL addr
            (0x2, _, _, _) => {
                self.stack.push(self.memory.pc_register);
                self.memory.pc_register = instruction.nnn;
                jump = true;
            }
            // 3xkk - SE Vx, byte
            (0x3, _, _, _) => {
                if v_x == instruction.kk {
                    self.memory.increase_pc();
                }
            }
            // 4xkk - SNE Vx, byte
            (0x4, _, _, _) => {
                if v_x != instruction.kk {
                    self.memory.increase_pc();
                }
            }
            // 5xy0 - SE Vx, Vy
            (0x5, _, _, 0x0) => {
                if v_x == v_y {
                    self.memory.increase_pc();
                }
            }
            // 6xkk - LD Vx, byte
            (0x6, _, _, _) => self.v_registers[instruction.x] = instruction.kk,
            // 7xkk - ADD Vx, byte
            (0x7, _, _, _) => self.v_registers[instruction.x] = v_x.wrapping_add(instruction.kk),
            // 8xy0 - LD Vx, Vy
            (0x8, _, _, 0x0) => self.v_registers[instruction.x] = v_y,

            // 8xy1 - OR Vx, Vy
            (0x8, _, _, 0x1) => self.v_registers[instruction.x] |= v_y,
            // 8xy2 - AND Vx, Vy
            (0x8, _, _, 0x2) => self.v_registers[instruction.x] &= v_y,
            // 8xy3 - XOR Vx, Vy
            (0x8, _, _, 0x3) => self.v_registers[instruction.x] ^= v_y,
            // 8xy4 - ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                self.set_flag(u16::from(v_x) + u16::from(v_y) > 255);
                self.v_registers[instruction.x] = v_x.wrapping_add(v_y);
            }
            // 8xy5 - SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                self.set_flag(v_x > v_y);
                self.v_registers[instruction.x] = v_x.wrapping_sub(v_y);
            }
            // 8xy6 - SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                self.set_flag(v_x & 0x01 == 1);
                self.v_registers[instruction.x] = v_x >> 1;
            }
            // 8xy7 - SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                self.set_flag(v_y > v_x);
                self.v_registers[instruction.x] = v_y.wrapping_sub(v_x);
            }
            // 8xyE - SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                self.set_flag(v_x >> 7 == 1);
                self.v_registers[instruction.x] = v_x << 1;
            }
            // 9xy0 - SNE Vx, Vy
            (0x9, _, _, 0x0) => {
                if v_x != v_y {
                    self.memory.increase_pc();
                }
            }
            // Annn - LD I, addr
            (0xA, _, _, _) => self.memory.i_register = instruction.nnn,
            // Bnnn - JP V0, addr
            (0xB, _, _, _) => {
                self.memory.pc_register = instruction.nnn + u16::from(self.v_registers[0x0]);
                jump = true;
            }
            // Cxkk - RND Vx, byte
            (0xC, _, _, _) => {
                self.v_registers[instruction.x] = thread_rng().gen::<u8>() & instruction.kk;
            }
            // Dxyn - DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let sprite = self.memory.get_bytes(instruction.n);
                let colision = self.display.draw_sprite(sprite, v_x, v_y);
                self.set_flag(colision);
            }
            // Ex9E - SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if self.keyboard.is_key_pressed(v_x) {
                    self.memory.increase_pc();
                }
            }
            // ExA1 - SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if !self.keyboard.is_key_pressed(v_x) {
                    self.memory.increase_pc();
                }
            }
            // Fx07 - LD Vx, DT
            (0xF, _, 0x0, 0x7) => {
                self.v_registers[instruction.x] = self.delay_timer.get_remaining();
            }
            // Fx0A - LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                if let Some(key) = self.keyboard.get_key() {
                    self.v_registers[instruction.x] = key;
                }
            }
            // Fx15 - LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.delay_timer.set_time(v_x),
            // Fx18 - LD ST, Vx
            (0xF, _, 0x1, 0x8) => self.sound_timer.set_time(v_x),
            // Fx1E - ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.memory.i_register += u16::from(v_x),
            // Fx29 - LD F, Vx
            (0xF, _, 0x2, 0x9) => self.memory.load_default_sprite(v_x),
            // Fx33 - LD B, Vx
            (0xF, _, 0x3, 0x3) => self.memory.load_decimal_to_memory(v_x),
            // Fx55 - LD [I], Vx
            (0xF, _, 0x5, 0x5) => {
                let bytes = &self.v_registers[0..instruction.x];
                self.memory.load_bytes_to_memory(bytes);
            }
            // Fx65 - LD Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                let bytes = self.memory.get_bytes(1 + instruction.x as u8);
                self.v_registers[0..=instruction.x].copy_from_slice(bytes);
            }
            _ => panic!("Unknown instruction"),
        };

        if !self.keyboard.is_waiting() && !jump {
            self.memory.increase_pc();
        }
    }
}
