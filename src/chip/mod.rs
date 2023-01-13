mod display;
mod keyboard;
mod timer;

use display::Display;
use keyboard::Keyboard;
use timer::{Delay, Sound};

const MAX_MEMORY: usize = 4096;
const ROM_START: u16 = 0x200;
const REGISTERS_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

const DEFAULT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// s or start - A 4-bit value, the first 4 bits of the instruction
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
enum BitVariables {
    S,
    Nnn,
    N,
    X,
    Y,
    KK,
}

pub struct Chip {
    memory: [u8; MAX_MEMORY],
    v_registers: [u8; REGISTERS_COUNT],
    i_register: u16,
    program_counter_register: u16,
    stack_pointer_register: u8,
    delay_timer: Delay,
    sound_timer: Sound,
    stack: [u16; STACK_SIZE],
    keyboard: Keyboard,
    display: Display,
}

impl Chip {
    pub fn new() -> Chip {
        let mut chip = Chip {
            memory: [0; MAX_MEMORY],
            v_registers: [0; REGISTERS_COUNT],
            i_register: 0,
            program_counter_register: 0,
            stack_pointer_register: 0,
            delay_timer: Delay::default(),
            sound_timer: Sound::default(),
            stack: [0; STACK_SIZE],
            keyboard: Keyboard::default(),
            display: Display::new(),
        };

        for (i, byte) in DEFAULT_SPRITES.iter().enumerate() {
            chip.memory[i] = *byte;
        }

        chip.program_counter_register = 0x200;

        chip
    }

    // Returns the part of the u16 represented by the variable
    fn resolve_pc_variable(&self, variable: BitVariables) -> u16 {
        let pc = self.program_counter_register;
        match variable {
            BitVariables::S => pc >> 12,
            BitVariables::Nnn => pc & 0x0FFF,
            BitVariables::N => pc & 0x000F,
            BitVariables::X => (pc & 0x0F00) >> 8,
            BitVariables::Y => (pc & 0x00F0) >> 4,
            BitVariables::KK => pc & 0x00FF,
        }
    }

    fn process_instruction(&mut self) {
        let nibbles: (u8, u8, u8, u8) = (
            self.resolve_pc_variable(BitVariables::S) as u8,
            self.resolve_pc_variable(BitVariables::X) as u8,
            self.resolve_pc_variable(BitVariables::Y) as u8,
            self.resolve_pc_variable(BitVariables::N) as u8,
        );

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
