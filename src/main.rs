use clap::Parser;
use image::io::Reader as ImageReader;

pub mod desmos_output;
pub mod edge_detection;
pub mod equation_generator;
pub mod utils;
use utils::*;
pub mod pipeline;
pub mod samplers;
use pipeline::{FunctionTransform, Pipeline};

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

use samplers::{Sampler, UniformSpacialSampler};

fn main() {
    let args = Args::parse();

    if args.frequency_count > args.sample_count {
        eprintln!("warning: Frequency is higher than samples. This may lead to aliasing");
    }

    let contour = FunctionTransform::new(|filename: &str| {
        ImageReader::open(filename)
            .expect("fail to open image")
            .decode()
            .expect("failed to decode image")
    }, "opening image").compose(FunctionTransform::new(|image: image::DynamicImage| {
        edge_detection::get_edges(image, &args)
    }, "detecting edges")).compose(FunctionTransform::new(|x| {
        let mut contour = edge_detection::construct_contour(x);
        contour.push(contour[0]);
        contour
    }, "constructing contour")).compose(FunctionTransform::new(|x: Vec<Vector2>| {
        UniformSpacialSampler::new(x).sample(args.sample_count)
    }, "sampling contour"));

    if args.table {
        let pipeline = contour.compose(FunctionTransform::new(|x: Vec<Vector2>| {
            desmos_output::table(&x)
        }, "serializing data"));
        println!("{}", pipeline.exec_pipeline(&args.image, &args));
    } else {
        let pipeline = contour.compose(FunctionTransform::new(|x: Vec<Vector2>| {
            equation_generator::get_frequency_info(&x, &args)
        }, "calculating frequencies")).compose(FunctionTransform::new(|x: Vec<(i32, Vector2)>| {
            desmos_output::parametric_sin(&x)
        }, "serializing data"));
        println!("{}", pipeline.exec_pipeline(&args.image, &args));
    }
}
