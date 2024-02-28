use std::time::{Duration, Instant};

const TIMER_TICK: Duration = Duration::from_micros(1_000_000 / 60);

pub struct Timer {
    register: u8,
    start_tick_time: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            register: Default::default(),
            start_tick_time: Instant::now(),
        }
    }
}

impl Timer {
    pub fn countdown(&mut self) -> bool {
        if self.register > 0 && self.start_tick_time.elapsed() >= TIMER_TICK {
            self.register -= 1;
            self.start_tick_time = Instant::now();
            return true;
        }

        false
    }

    pub fn set_time(&mut self, time: u8) {
        self.register = time;

        self.start_tick_time = Instant::now();
    }

    pub const fn get_remaining(&self) -> u8 {
        self.register
    }
}

#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
    waiting_input: bool,
    last_pressed: Option<u8>,
}

pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
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

    pub const fn is_key_pressed(&self, key: u8) -> bool {
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

    pub const fn is_waiting(&self) -> bool {
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
        assert!(keyboard.waiting_input);
    }

    #[test]
    fn test_get_key_not_pressed() {
        let mut keyboard = Keyboard {
            waiting_input: true,
            ..Default::default()
        };

        let key = keyboard.get_key();

        assert_eq!(key, None);
        assert!(keyboard.waiting_input);
    }

    #[test]
    fn test_get_key_pressed() {
        let mut keyboard = Keyboard {
            last_pressed: Some(KEY),
            waiting_input: true,
            ..Default::default()
        };

        let key = keyboard.get_key();

        assert_eq!(key, Some(KEY));
        assert!(!keyboard.waiting_input);
        assert_eq!(keyboard.last_pressed, None);
    }
}
