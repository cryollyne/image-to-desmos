use std::cmp::Ordering;

use crate::Vector2;

use super::Sampler;

#[derive(Debug)]
pub struct UniformSpacialSampler {
    points: Vec<Vector2>,
    cumulative_distance: Vec<f64>,
    total_distance: f64,
}

impl Sampler for UniformSpacialSampler {
    fn new(points: Vec<Vector2>) -> Self {
        let distances = points.iter()
            .zip(points[1..].iter())
            .map(|(a, b)| {
                (*a-*b).magnitude()
            }).collect::<Vec<f64>>();

        let total_distance = distances.iter().sum();

        let mut cumulative_distance = vec![0f64];
        for dist in distances {
            let next_distance = dist + cumulative_distance.last().unwrap();
            cumulative_distance.push(next_distance)
        }

        UniformSpacialSampler {
            points,
            total_distance,
            cumulative_distance,
        }
    }

    fn sample_point(&self, t: f64) -> Vector2 {
        let dist = t * self.total_distance;
        let prev_index = match self.cumulative_distance.binary_search_by(|x| {
            if *x > dist {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) {
            Ok(i) => i,
            Err(i) => i-1,
        };
        let prev = self.points[prev_index];
        let next = *self.points.get(prev_index+1).unwrap_or(&prev);

        let segment_distance = (next - prev).magnitude();
        let t2 = if segment_distance == 0.0 {
            0.0
        } else {
            (dist - self.cumulative_distance[prev_index]) / segment_distance
        };

        prev*(1.0-t2) + next*t2
    }
}
