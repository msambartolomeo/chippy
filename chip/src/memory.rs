const MAX_MEMORY: usize = 4096;
const DEFAULT_SPRITE_SIZE: usize = 5;

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
    pub fn new(start: u16) -> Memory {
        let mut memory = Memory {
            array: [0; MAX_MEMORY],
            i_register: 0,
            pc_register: start,
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

        self.load_bytes_to_memory(&[hundreds, tens, ones]);
    }

    pub fn load_bytes_to_memory(&mut self, bytes: &[u8]) {
        let index = self.i_register as usize;
        self.array[index..index + bytes.len()].copy_from_slice(bytes)
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

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
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

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_START: u16 = 0x200;

    #[test]
    fn test_instruction() {
        let instruction = Instruction::from(0x1234);

        assert_eq!(instruction.s, 1);
        assert_eq!(instruction.x, 2);
        assert_eq!(instruction.y, 3);
        assert_eq!(instruction.n, 4);
        assert_eq!(instruction.kk, 0x34);
        assert_eq!(instruction.nnn, 0x234);
    }

    #[test]
    fn test_nibbles() {
        let instruction = Instruction::from(0x1234);

        assert_eq!((1, 2, 3, 4), instruction.get_nibbles());
    }

    #[test]
    fn test_new_memory() {
        let memory = Memory::new(DEFAULT_START);

        assert_eq!(memory.array[0..DEFAULT_SPRITES.len()], DEFAULT_SPRITES);
    }

    #[test]
    fn test_load_default_sprite() {
        let mut memory = Memory::new(DEFAULT_START);

        memory.load_default_sprite(0xF);

        let i = memory.i_register as usize;
        assert_eq!(
            memory.array[i..i + DEFAULT_SPRITE_SIZE],
            DEFAULT_SPRITES[75..80]
        )
    }

    #[test]
    fn test_load_decimal() {
        let mut memory = Memory::new(DEFAULT_START);
        memory.i_register = DEFAULT_START;

        memory.load_decimal_to_memory(123);

        let i = memory.i_register as usize;
        assert_eq!(memory.array[i..i + 3], [1, 2, 3]);
    }

    #[test]
    fn test_get_bytes() {
        let mut memory = Memory::new(DEFAULT_START);
        memory.i_register = 0;
        let bytes = memory.get_bytes(10);

        assert_eq!(&DEFAULT_SPRITES[0..10], bytes);
    }

    #[test]
    fn test_load_bytes() {
        let mut memory = Memory::new(DEFAULT_START);
        memory.i_register = DEFAULT_START;
        const DATA: [u8; 3] = [1, 2, 3];

        memory.load_bytes_to_memory(&DATA);

        let i = memory.i_register as usize;
        assert_eq!(memory.array[i..i + DATA.len()], DATA);
    }

    #[test]
    fn test_increase_pc() {
        let mut memory = Memory::new(DEFAULT_START);

        memory.increase_pc();

        assert_eq!(memory.pc_register, DEFAULT_START + 2);
    }

    #[test]
    fn test_get_instruction() {
        let mut memory = Memory::new(DEFAULT_START);
        let pc = memory.pc_register as usize;
        memory.array[pc] = 0x12;
        memory.array[pc + 1] = 0x34;

        let instruction = memory.get_current_instruction();

        assert_eq!(instruction, Instruction::from(0x1234));
    }
}
