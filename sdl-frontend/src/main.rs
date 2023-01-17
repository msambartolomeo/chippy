use chip::Chip;

fn main() {
    let mut chip: Chip = Chip::default();

    loop {
        chip.run_cycle();
    }
}
