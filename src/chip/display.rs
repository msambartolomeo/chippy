const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    screen: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    // NOTE: Returns true on colision
    pub fn draw_sprite(&mut self, sprite: &[u8], x: u8, y: u8) -> bool {
        if sprite.len() > 15 {
            panic!("Unsupported sprite");
        }

        let x = (x % 64) as usize;
        let y = (y % 64) as usize;

        let mut colision = false;

        for (height, byte) in sprite.iter().enumerate() {
            for bit in 0..8 {
                let old_pixel = &mut self.screen[x + bit][y + height];
                let new_pixel = byte & (1 << bit) != 0;

                if !colision && *old_pixel && new_pixel {
                    colision = true;
                }

                *old_pixel = *old_pixel != new_pixel;
            }
        }

        colision
    }
}
