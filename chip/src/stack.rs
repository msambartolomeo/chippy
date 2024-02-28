const STACK_SIZE: usize = 16;

#[derive(Default)]
pub struct Stack {
    array: [u16; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    pub fn push(&mut self, value: u16) {
        assert!(self.stack_pointer < STACK_SIZE, "Stack is full");
        self.array[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        assert!(self.stack_pointer != 0, "Stack is empty");

        self.stack_pointer -= 1;

        self.array[self.stack_pointer]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_capacity() {
        let mut stack = Stack::default();

        for _ in 0..STACK_SIZE {
            stack.push(1);
        }

        for _ in 0..STACK_SIZE {
            stack.pop();
        }
    }

    #[test]
    #[should_panic(expected = "Stack is full")]
    fn test_stack_full() {
        let mut stack = Stack::default();

        for _ in 0..=STACK_SIZE {
            stack.push(1);
        }
    }

    #[test]
    #[should_panic(expected = "Stack is empty")]
    fn test_stack_empty() {
        let mut stack = Stack::default();

        stack.pop();
    }
}
