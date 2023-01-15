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
