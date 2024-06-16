use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    image: String,
}

fn main() {
    let args = Args::parse();
}
