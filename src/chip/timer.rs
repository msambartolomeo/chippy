pub trait Timer {
    fn countdown(&mut self);
    fn set_timer(&mut self, time: u8);
}

#[derive(Default)]
pub struct Sound {
    register: u8,
}

#[derive(Default)]
pub struct Delay {
    register: u8,
}

impl Timer for Sound {
    fn countdown(&mut self) {
        if self.register > 0 {
            self.register -= 1;
            // TODO: Beep
            println!("Beep");
        }
    }

    fn set_timer(&mut self, time: u8) {
        self.register = time;
    }
}

impl Timer for Delay {
    fn countdown(&mut self) {
        if self.register > 0 {
            self.register -= 1;
        }
    }

    fn set_timer(&mut self, time: u8) {
        self.register = time;
    }
}
