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
        let nibbles = (
            self.resolve_pc_variable(BitVariables::S),
            self.resolve_pc_variable(BitVariables::X),
            self.resolve_pc_variable(BitVariables::Y),
            self.resolve_pc_variable(BitVariables::N),
        );

        match nibbles {
            (0, 0, 0xE, 0) => self.display.clear(),
            _ => todo!(),
        }
    }
}
