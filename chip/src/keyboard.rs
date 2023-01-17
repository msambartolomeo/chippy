#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
    waiting_input: bool,
    last_pressed: Option<u8>,
}

pub enum Key {
    Key1,
    Key2,
    Key3,
    KeyC,
    Key4,
    Key5,
    Key6,
    KeyD,
    Key7,
    Key8,
    Key9,
    KeyE,
    KeyA,
    Key0,
    KeyB,
    KeyF,
}

impl Keyboard {
    pub fn press_key(&mut self, key: Key) {
        let key = key as usize;
        self.keys[key] = true;

        if self.waiting_input {
            self.last_pressed = Some(key as u8);
        }
    }

    pub fn unpress_key(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn get_key(&mut self) -> Option<u8> {
        if self.waiting_input && self.last_pressed.is_some() {
            self.waiting_input = false;

            self.last_pressed.take()
        } else {
            self.waiting_input = true;

            None
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.waiting_input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: u8 = 0xF;

    #[test]
    fn test_get_key_not_waiting() {
        let mut keyboard = Keyboard::default();

        let key = keyboard.get_key();

        assert_eq!(key, None);
        assert_eq!(keyboard.waiting_input, true);
    }

    #[test]
    fn test_get_key_not_pressed() {
        let mut keyboard = Keyboard::default();
        keyboard.waiting_input = true;

        let key = keyboard.get_key();

        assert_eq!(key, None);
        assert_eq!(keyboard.waiting_input, true);
    }

    #[test]
    fn test_get_key_pressed() {
        let mut keyboard = Keyboard::default();
        keyboard.waiting_input = true;
        keyboard.last_pressed = Some(KEY);

        let key = keyboard.get_key();

        assert_eq!(key, Some(KEY));
        assert_eq!(keyboard.waiting_input, false);
        assert_eq!(keyboard.last_pressed, None);
    }
}
