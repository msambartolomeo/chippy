const MAX_MEMORY: usize = 4096;
const DEFAULT_SPRITE_SIZE: usize = 5;
const ROM_START: u16 = 0x200;

const DEFAULT_SPRITES: [u8; DEFAULT_SPRITE_SIZE * 16] = [
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

pub struct Memory {
    array: [u8; MAX_MEMORY],
    pub i_register: u16,
    pub pc_register: u16,
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            array: [0; MAX_MEMORY],
            i_register: 0,
            pc_register: ROM_START,
        };

        memory.array[0..DEFAULT_SPRITES.len()].copy_from_slice(&DEFAULT_SPRITES);

        memory
    }

    pub fn get_bytes(&self, count: u8) -> &[u8] {
        let count = count as usize;
        let i = self.i_register as usize;
        &self.array[i..i + count]
    }

    pub fn get_current_instruction(&self) -> Instruction {
        let address = self.pc_register as usize;
        let first = self.array[address] as u16;
        let second = self.array[address + 1] as u16;

        let raw_instruction = first << 8 | second;

        Instruction::from(raw_instruction)
    }

    pub fn increase_pc(&mut self) {
        self.pc_register += 2;
    }

    pub fn load_default_sprite(&mut self, x: u8) {
        if x > 0xF {
            panic!("Invalid default sprite");
        }

        self.i_register = x as u16 * DEFAULT_SPRITE_SIZE as u16;
    }

    pub fn load_decimal_to_memory(&mut self, num: u8) {
        let hundreds = num / 100;
        let tens = num / 10 % 10;
        let ones = num % 10;

        let i = self.i_register as usize;

        self.array[i] = hundreds;
        self.array[i + 1] = tens;
        self.array[i + 2] = ones;
    }

    pub fn load_bytes_to_memory(&mut self, bytes: &[u8]) {
        let index = self.i_register as usize;
        self.array[index..bytes.len()].copy_from_slice(bytes)
    }
}

// s or start - A 4-bit value, the first 4 bits of the instruction
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
pub enum BitVariables {
    S,
    Nnn,
    N,
    X,
    Y,
    Kk,
}

// Returns the part of the u16 represented by the variable
fn get_variable(value: u16, variable: BitVariables) -> u16 {
    match variable {
        BitVariables::S => value >> 12,
        BitVariables::Nnn => value & 0x0FFF,
        BitVariables::N => value & 0x000F,
        BitVariables::X => (value & 0x0F00) >> 8,
        BitVariables::Y => (value & 0x00F0) >> 4,
        BitVariables::Kk => value & 0x00FF,
    }
}

pub struct Instruction {
    pub s: u8,
    pub x: usize,
    pub y: usize,
    pub n: u8,
    pub nnn: u16,
    pub kk: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Instruction {
            s: get_variable(value, BitVariables::S) as u8,
            x: get_variable(value, BitVariables::X) as usize,
            y: get_variable(value, BitVariables::Y) as usize,
            n: get_variable(value, BitVariables::N) as u8,
            nnn: get_variable(value, BitVariables::Nnn),
            kk: get_variable(value, BitVariables::Kk) as u8,
        }
    }
}

impl Instruction {
    pub fn get_nibbles(&self) -> (u8, u8, u8, u8) {
        (self.s, self.x as u8, self.y as u8, self.n)
    }
}
