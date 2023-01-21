mod args;
mod display;

use args::Args;
use chip::{Chip, Key};
use display::Display;

use clap::Parser;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn get_key(scancode: Option<Scancode>) -> Option<Key> {
    match scancode {
        Some(Scancode::Num1) => Some(Key::Key1),
        Some(Scancode::Num2) => Some(Key::Key2),
        Some(Scancode::Num3) => Some(Key::Key3),
        Some(Scancode::Num4) => Some(Key::KeyC),
        Some(Scancode::Q) => Some(Key::Key4),
        Some(Scancode::W) => Some(Key::Key5),
        Some(Scancode::E) => Some(Key::Key6),
        Some(Scancode::R) => Some(Key::KeyD),
        Some(Scancode::A) => Some(Key::Key7),
        Some(Scancode::S) => Some(Key::Key8),
        Some(Scancode::D) => Some(Key::Key9),
        Some(Scancode::F) => Some(Key::KeyE),
        Some(Scancode::Z) => Some(Key::KeyA),
        Some(Scancode::X) => Some(Key::Key0),
        Some(Scancode::C) => Some(Key::KeyB),
        Some(Scancode::V) => Some(Key::KeyF),

        _ => None,
    }
}

fn main() {
    let rom_path = Args::parse().path;

    let mut chip: Chip = Chip::default();
    chip.load_rom(rom_path).expect("Valid_rom");

    let sdl = sdl2::init().expect("Sdl creation error");

    let mut display = Display::init(&sdl).expect("Screen initialization error");

    let mut events = sdl.event_pump().expect("event pump creation error");

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { scancode, .. } => {
                    if let Some(key) = get_key(scancode) {
                        chip.press_key(key);
                    }
                }
                Event::KeyUp { scancode, .. } => {
                    if let Some(key) = get_key(scancode) {
                        chip.unpress_key(key);
                    }
                }
                _ => (),
            }
        }

        let actions = chip.run_cycle();

        if actions.draw {
            display
                .draw(chip.screen())
                .expect("Error drawing to screen");
        }
    }

    println!();
}
