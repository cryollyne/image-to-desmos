mod uniform_spatial_sampler;
mod uniform_temporal_sampler;

pub use uniform_temporal_sampler::UniformTemporalSampler;
pub use uniform_spatial_sampler::UniformSpacialSampler;

use crate::Vector2;

pub trait Sampler : Sized {
    fn new(points: Vec<Vector2>) -> Self;
    fn sample_point(&self, t: f64) -> Vector2;
    fn sample(&self, samples: u32) -> Vec<Vector2> {
        (0..samples)
            .map(|i| {self.sample_point(i as f64 / samples as f64)})
            .collect::<Vec<Vector2>>()
    }
}
