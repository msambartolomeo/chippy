use clap::Parser;

#[derive(Parser)]
#[command(name = "Chippy")]
#[command(author = "Mauro Sambartolomeo")]
#[command(version = "1.0")]
#[command(about = "Chip-8 interpreter written in rust")]
pub struct Args {
    pub path: String,
}
