mod display;
mod keyboard;
mod memory;
mod stack;
mod timer;

use display::Display;
use keyboard::Keyboard;
use memory::Instruction;
use memory::Memory;
use rand::Rng;
use stack::Stack;
use timer::{Delay, Sound};

use rand::thread_rng;

const ROM_START: u16 = 0x200;
const REGISTERS_COUNT: usize = 16;

pub struct Chip {
    v_registers: [u8; REGISTERS_COUNT],
    i_register: u16,
    pc_register: u16,
    memory: Memory,
    delay_timer: Delay,
    sound_timer: Sound,
    stack: Stack,
    keyboard: Keyboard,
    display: Display,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            v_registers: [0; REGISTERS_COUNT],
            i_register: 0,
            pc_register: ROM_START,
            memory: Memory::new(),
            delay_timer: Delay::default(),
            sound_timer: Sound::default(),
            stack: Stack::new(),
            keyboard: Keyboard::default(),
            display: Display::new(),
        }
    }

    #[inline]
    fn increase_pc(&mut self) {
        self.pc_register += 2;
    }

    #[inline]
    fn set_flag(&mut self, condition: bool) {
        self.v_registers[0xF] = match condition {
            true => 1,
            false => 0,
        };
    }

    fn process_instruction(&mut self) {
        let instruction = Instruction::from(self.pc_register);

        let nibbles = instruction.get_nibbles();

        let v_x = self.v_registers[instruction.x];
        let v_y = self.v_registers[instruction.y];

        match nibbles {
            // 00E0 - CLS
            (0x0, 0x0, 0xE, 0x0) => self.display.clear(),
            // 00EE - RET
            (0x0, 0x0, 0xE, 0xE) => self.pc_register = self.stack.pop(),
            // 0nnn - SYS addr - Ignored
            (0x0, _, _, _) => (),
            // 1nnn - JP addr
            (0x1, _, _, _) => self.pc_register = instruction.nnn,
            // 2nnn - CALL addr
            (0x2, _, _, _) => {
                self.stack.push(self.pc_register);
                self.pc_register = instruction.nnn;
            }
            // 3xkk - SE Vx, byte
            (0x3, _, _, _) => {
                if v_x == instruction.kk {
                    self.increase_pc();
                }
            }
            // 4xkk - SNE Vx, byte
            (0x4, _, _, _) => {
                if v_x != instruction.kk {
                    self.increase_pc();
                }
            }
            // 5xy0 - SE Vx, Vy
            (0x5, _, _, 0x0) => {
                if v_x == v_y {
                    self.increase_pc();
                }
            }
            // 6xkk - LD Vx, byte
            (0x6, _, _, _) => self.v_registers[instruction.x] = instruction.kk,
            // 7xkk - ADD Vx, byte
            (0x7, _, _, _) => self.v_registers[instruction.x] += instruction.kk,
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
                self.set_flag(v_x as u16 + v_y as u16 > 255);
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
            (0x9, _, _, 0x0) => todo!(),
            // Annn - LD I, addr
            (0xA, _, _, _) => todo!(),
            // Bnnn - JP V0, addr
            (0xB, _, _, _) => todo!(),
            // Cxkk - RND Vx, byte
            (0xC, _, _, _) => {
                self.v_registers[instruction.x] = thread_rng().gen::<u8>() & instruction.kk
            }
            // Dxyn - DRW Vx, Vy, nibble
            (0xD, _, _, _) => todo!(),
            // Ex9E - SKP Vx
            (0xE, _, 0x9, 0xE) => todo!(),
            // ExA1 - SKNP Vx
            (0xE, _, 0xA, 0x1) => todo!(),
            // Fx07 - LD Vx, DT
            (0xF, _, 0x0, 0x7) => todo!(),
            // Fx0A - LD Vx, K
            (0xF, _, 0x0, 0xA) => todo!(),
            // Fx15 - LD DT, Vx
            (0xF, _, 0x1, 0x5) => todo!(),
            // Fx18 - LD ST, Vx
            (0xF, _, 0x1, 0x8) => todo!(),
            // Fx1E - ADD I, Vx
            (0xF, _, 0x1, 0xE) => todo!(),
            // Fx29 - LD F, Vx
            (0xF, _, 0x2, 0x9) => todo!(),
            // Fx33 - LD B, Vx
            (0xF, _, 0x3, 0x3) => todo!(),
            // Fx55 - LD [I], Vx
            (0xF, _, 0x5, 0x5) => todo!(),
            // Fx65 - LD Vx, [I]
            (0xF, _, 0x6, 0x5) => todo!(),
            _ => panic!("Unknown instruction"),
        };
    }
}
