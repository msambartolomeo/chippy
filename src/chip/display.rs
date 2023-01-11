const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    screen: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }
}
