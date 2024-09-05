use crate::Vector2;

use super::Sampler;

#[derive(Debug)]
pub struct UniformTemporalSampler {
    points: Vec<Vector2>,
}

impl Sampler for UniformTemporalSampler {
    fn new(points: Vec<Vector2>) -> Self {
        UniformTemporalSampler {
            points
        }
    }

    fn sample_point(&self, t: f64) -> Vector2 {
        let location = (self.points.len()-1) as f64 * t;
        let prev = self.points[f64::floor(location) as usize];
        let next = self.points[f64::ceil(location) as usize];
        let t2 = f64::fract(location);
        (1.0-t2) * prev + t2 * next
    }
}
