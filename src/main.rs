use clap::Parser;
pub mod utils;
pub mod equation_generator;
pub mod desmos_output;
use utils::*;


#[derive(Parser, Debug)]
struct Args {
    image: String,
}

fn main() {
    let args = Args::parse();
}
