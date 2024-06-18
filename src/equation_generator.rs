use crate::{utils::*, Args};
pub fn sample(points: &[Vector2], t: f64) -> Vector2 {
    let location = (points.len()-1) as f64 * t;
    let prev = points[f64::floor(location) as usize];
    let next = points[f64::ceil(location) as usize];
    let t2 = f64::fract(location);
    (1.0-t2) * prev + t2 * next
}

fn get_magnitude_of_frequency(points: &[Complex], frequence: i32) -> Complex {
    (0..points.len()).map(|i| {
        points[i] * Complex::from_polar(1.0, -2.0*std::f64::consts::PI*(i as f64)*(frequence as f64)/(points.len() as f64))
    }).reduce(|acc, x| {
        acc + x
    }).unwrap()
}

pub fn get_frequency_info(points: &[Vector2], args: &Args) -> Vec<(i32, Vector2)> {
    if args.verbose {
        eprintln!("[5/5] calculating frequencies");
    }

    let p = points.iter().map(|x| {Complex::from_vector2(*x)}).collect::<Vec<Complex>>();
    let size = points.len() as i32;
    ((-size/2+1)..(size/2))
        .map(|i| {
            (i, get_magnitude_of_frequency(&p, i) * Complex::from_real(1.0/(points.len() as f64)))
        }).map(|(i, x)| {(i, Vector2::from_complex(x))})
        .collect()
}
