pub trait Timer {
    fn countdown(&mut self);
}

#[derive(Default)]
pub struct Sound {
    pub register: u8,
    beep: bool,
}

#[derive(Default)]
pub struct Delay {
    pub register: u8,
}

impl Sound {
    pub fn must_beep(&mut self) -> bool {
        if self.beep {
            self.beep = false;

            return true;
        }

        false
    }
}

impl Timer for Sound {
    fn countdown(&mut self) {
        if self.register > 0 {
            self.register -= 1;
            self.beep = true;
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
