const STACK_SIZE: usize = 16;

#[derive(Default)]
pub struct Stack {
    array: [u16; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    pub fn push(&mut self, value: u16) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("Stack is full");
        }
        self.array[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.stack_pointer == 0 {
            panic!("Stack is empty");
        }

        let value = self.array[self.stack_pointer];
        self.stack_pointer -= 1;

        value
    }
}
