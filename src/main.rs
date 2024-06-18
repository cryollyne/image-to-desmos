use clap::Parser;
use image::io::Reader as ImageReader;

pub mod utils;
pub mod equation_generator;
pub mod desmos_output;
pub mod edge_detection;
use utils::*;


#[derive(Parser, Debug)]
pub struct Args {
    image: String,

    #[arg(long)]
    debug_output_image: Option<String>,

    #[arg(long, default_value_t = 1.0)]
    edge_threshold: f64,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        eprintln!("opening image");
    }
    let image = ImageReader::open(&args.image).expect("fail to open image")
        .decode().expect("failed to decode image");

    if args.verbose {
        eprintln!("filtering edge points");
    }
    let edge_points = edge_detection::get_edges(image, &args);

    let contour = edge_detection::construct_contour(edge_points, args);
}
