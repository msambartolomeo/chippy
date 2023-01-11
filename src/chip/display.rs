pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
}
