const MAX_MEMORY: usize = 4096;

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

pub struct Memory {
    array: [u8; MAX_MEMORY],
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            array: [0; MAX_MEMORY],
        };

        for (i, byte) in DEFAULT_SPRITES.iter().enumerate() {
            memory.array[i] = *byte;
        }

        memory
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
    value: u16,
    pub s: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nnn: u16,
    pub kk: u16,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Instruction {
            value,
            s: get_variable(value, BitVariables::S) as u8,
            x: get_variable(value, BitVariables::X) as u8,
            y: get_variable(value, BitVariables::Y) as u8,
            n: get_variable(value, BitVariables::N) as u8,
            nnn: get_variable(value, BitVariables::Nnn),
            kk: get_variable(value, BitVariables::Kk),
        }
    }
}

impl Instruction {
    pub fn get_nibbles(&self) -> (u8, u8, u8, u8) {
        (self.s, self.x, self.y, self.n)
    }
}
