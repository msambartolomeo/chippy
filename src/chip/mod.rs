mod display;
mod keyboard;
mod memory;
mod stack;
mod timer;

use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use stack::Stack;
use timer::{Delay, Sound};

use self::memory::Instruction;

const ROM_START: u16 = 0x200;
const REGISTERS_COUNT: usize = 16;

pub struct Chip {
    v_registers: [u8; REGISTERS_COUNT],
    i_register: u16,
    program_counter_register: u16,
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
            program_counter_register: ROM_START,
            memory: Memory::new(),
            delay_timer: Delay::default(),
            sound_timer: Sound::default(),
            stack: Stack::new(),
            keyboard: Keyboard::default(),
            display: Display::new(),
        }
    }

    fn process_instruction(&mut self) {
        let instruction = Instruction::from(self.program_counter_register);

        let nibbles = instruction.get_nibbles();

        match nibbles {
            // 00E0 - CLS
            (0x0, 0x0, 0xE, 0x0) => self.display.clear(),
            // 00EE - RET
            (0x0, 0x0, 0xE, 0xE) => todo!(),
            // 0nnn - SYS addr - Ignored
            (0x0, _, _, _) => (),
            // 1nnn - JP addr
            (0x1, _, _, _) => todo!(),
            // 2nnn - CALL addr
            (0x2, _, _, _) => todo!(),
            // 3xkk - SE Vx, byte
            (0x3, _, _, _) => todo!(),
            // 4xkk - SNE Vx, byte
            (0x4, _, _, _) => todo!(),
            // 5xy0 - SE Vx, Vy
            (0x5, _, _, 0x0) => todo!(),
            // 6xkk - LD Vx, byte
            (0x6, _, _, _) => todo!(),
            // 7xkk - ADD Vx, byte
            (0x7, _, _, _) => todo!(),
            // 8xy0 - LD Vx, Vy
            (0x8, _, _, 0x0) => todo!(),
            // 8xy1 - OR Vx, Vy
            (0x8, _, _, 0x1) => todo!(),
            // 8xy2 - AND Vx, Vy
            (0x8, _, _, 0x2) => todo!(),
            // 8xy3 - XOR Vx, Vy
            (0x8, _, _, 0x3) => todo!(),
            // 8xy4 - ADD Vx, Vy
            (0x8, _, _, 0x4) => todo!(),
            // 8xy5 - SUB Vx, Vy
            (0x8, _, _, 0x5) => todo!(),
            // 8xy6 - SHR Vx {, Vy}
            (0x8, _, _, 0x6) => todo!(),
            // 8xy7 - SUBN Vx, Vy
            (0x8, _, _, 0x7) => todo!(),
            // 8xyE - SHL Vx {, Vy}
            (0x8, _, _, 0xE) => todo!(),
            // 9xy0 - SNE Vx, Vy
            (0x9, _, _, 0x0) => todo!(),
            // Annn - LD I, addr
            (0xA, _, _, _) => todo!(),
            // Bnnn - JP V0, addr
            (0xB, _, _, _) => todo!(),
            // Cxkk - RND Vx, byte
            (0xC, _, _, _) => todo!(),
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
