use chip::Chip;

fn main() {
    let mut chip: Chip = Chip::new();

    loop {
        chip.run_cycle();
    }
}
