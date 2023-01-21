mod args;
mod display;
mod keyboard;

use args::Args;
use chip::Chip;
use display::Display;

use clap::Parser;
use sdl2::event::Event;

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
                _ => {}
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
