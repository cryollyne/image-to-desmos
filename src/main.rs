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

    #[arg(short, long, default_value_t = 16384)]
    sample_count: u32,

    #[arg(short, long, default_value_t = 512)]
    frequency_count: u32,

    #[arg(short, long, default_value_t = 0.2)]
    edge_threshold: f64,

    #[arg(long)]
    table: bool,


    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    debug_output_image: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.frequency_count > args.sample_count {
        eprintln!("warning: Frequency is higher than samples. This may lead to aliasing");
    }

    if args.verbose {
        eprintln!("[1/5] opening image");
    }
    let image = ImageReader::open(&args.image).expect("fail to open image")
        .decode().expect("failed to decode image");

    let edge_points = edge_detection::get_edges(image, &args);

    let contour = edge_detection::construct_contour(edge_points, &args);



    if args.verbose {
        eprintln!("[4/5] sampling contour");
    }

    let points = (0..args.sample_count).map(|i| {
        equation_generator::sample(&contour, i as f64/args.sample_count as f64)
    }).collect::<Vec<Vector2>>();

    if args.table {
        println!("{}", desmos_output::table(&points));
        return;
    }


    let f = equation_generator::get_frequency_info(&points, &args);
    println!("{}", desmos_output::parametric_sin(&f));
}
