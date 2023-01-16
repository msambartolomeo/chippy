pub trait Timer {
    fn countdown(&mut self);
}

#[derive(Default)]
pub struct Sound {
    pub register: u8,
}

#[derive(Default)]
pub struct Delay {
    pub register: u8,
}

impl Timer for Sound {
    fn countdown(&mut self) {
        if self.register > 0 {
            self.register -= 1;
            // TODO: Beep
            println!("Beep");
        }
    }
}

impl Timer for Delay {
    fn countdown(&mut self) {
        if self.register > 0 {
            self.register -= 1;
        }
    }
}
