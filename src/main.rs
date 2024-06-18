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

    #[arg(short, long, default_value_t = 256)]
    sample_count: u32,

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

    let contour = edge_detection::construct_contour(edge_points, &args);



    if args.verbose {
        eprintln!("sampling contour");
    }

    let points = (0..args.sample_count).map(|i| {
        equation_generator::sample(&contour, i as f64/args.sample_count as f64)
    }).collect::<Vec<Vector2>>();



    if args.verbose {
        eprintln!("getting frequencies");
    }

    let f = equation_generator::get_frequency_info(&points);
    println!("{}", desmos_output::parametric_sin(&f));
}
