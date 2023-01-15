#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
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
        self.keys[key as usize] = true;
    }

    pub fn unpress_key(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}
