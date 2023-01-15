mod chip;

use chip::Chip;

fn main() {
    let mut chip: Chip = Chip::new();

    chip.process_instruction();
}
