use crate::utils::*;

pub fn parametric_sin(frequencies: &[(i32,Vector2)]) -> String {
    let (x, y) = frequencies.iter().fold((String::new(), String::new()), |(x, y), (frequency, comp)| {
        (
            format!("{x}+({}cos{frequency}t-{}sin{frequency}t)", comp.x, comp.y),
            format!("{y}+({}sin{frequency}t+{}cos{frequency}t)", comp.x, comp.y)
        )
    });
    format!("({x},{y})")
}

pub fn table(points: &[Vector2]) -> String {
    points.iter().fold(String::new(), |acc, point| {
        format!("{acc}{}\t{}\n", point.x, point.y)
    })
}
