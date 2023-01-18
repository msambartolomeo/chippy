mod args;

use args::Args;
use chip::Chip;
use clap::Parser;

const SCALE: u32 = 10;

fn main() {
    let args = Args::parse();
    let rom_path = args.path;

    let mut chip: Chip = Chip::default();
    chip.load_rom(rom_path).expect("Valid_rom");

    let sdl = sdl2::init().expect("Sdl creation error");
    let video_subsystem = sdl.video().expect("Video subsystem creation error");
    let window = video_subsystem
        .window(
            "chippy",
            SCALE * chip::DISPLAY_WIDTH as u32,
            SCALE * chip::DISPLAY_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .expect("Window building error");

    let mut events = sdl.event_pump().expect("event pump creation error");

    'main: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        chip.run_cycle();
    }

    println!();
}
