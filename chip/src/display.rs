pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub type Screen = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

pub struct Display {
    screen: Screen,
    must_draw: bool,
}

impl Default for Display {
    fn default() -> Self {
        Display {
            screen: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            must_draw: false,
        }
    }
}

impl Display {
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn must_draw(&mut self) -> bool {
        if self.must_draw {
            self.must_draw = false;
            return true;
        }

        false
    }

    pub fn clear(&mut self) {
        self.screen = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.must_draw = true;
    }

    // NOTE: Returns true on colision
    pub fn draw_sprite(&mut self, sprite: &[u8], x: u8, y: u8) -> bool {
        if sprite.len() > 15 {
            panic!("Unsupported sprite");
        }

        let x = x as usize;
        let y = y as usize;

        let mut colision = false;

        for (height, byte) in sprite.iter().enumerate() {
            let row = self.screen.get_mut(y + height);
            if let Some(row) = row {
                for bit in 0..8 {
                    let old_pixel = row.get_mut(x + bit);
                    if let Some(old_pixel) = old_pixel {
                        let new_pixel = byte & (1 << (7 - bit)) != 0;

                        if !colision && *old_pixel && new_pixel {
                            colision = true;
                        }

                        *old_pixel = *old_pixel != new_pixel;
                    }
                }
            }
        }

        self.must_draw = true;

        colision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_empty() {
        let mut display = Display {
            screen: [[true; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            must_draw: true,
        };

        display.clear();

        assert_eq!(display.screen, [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])
    }

    const SPRITE: [u8; 3] = [0b00111111, 0b11111111, 0b11000010];
    const SPRITE_BOOL: [[bool; 8]; 3] = [
        [false, false, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, false, false, false, false, true, false],
    ];

    #[test]
    fn test_sprite_no_colision() {
        let mut display = Display::default();

        let colision = display.draw_sprite(&SPRITE, 0, 0);

        assert!(!colision);

        for height in 0..SPRITE.len() {
            for bit in 0..8 {
                assert_eq!(
                    display.screen[height][bit], SPRITE_BOOL[height][bit],
                    "{}, {}",
                    bit, height
                );
            }
        }
    }

    #[test]
    fn test_sprite_colision() {
        let mut display = Display::default();

        display.draw_sprite(&SPRITE, 0, 0);
        let colision = display.draw_sprite(&SPRITE, 7, 0);

        assert!(colision);

        let result = [true, false, true];
        for height in 0..SPRITE.len() {
            assert_eq!(display.screen[height][7], result[height]);
        }
    }
}
