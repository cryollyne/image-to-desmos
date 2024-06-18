use std::ops::{Mul, Div, Add, Sub};

use image::{Pixel, Rgba};

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn map(self, f: fn(f64) -> f64) -> Vector4 {
        Vector4 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            w: f(self.w),
        }
    }

    pub fn dot(self, rhs: Vector4) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z + self.w*rhs.w
    }

    pub fn from_color(pixel: Rgba<u8>) -> Vector4 {
        let channels = pixel.channels();
        Vector4 {
            x: channels[0] as f64 / 256.0,
            y: channels[1] as f64 / 256.0,
            z: channels[2] as f64 / 256.0,
            w: channels[3] as f64 / 256.0,
        }
    }

    pub fn as_array(self) -> [u8;4] {
        [
            (self.x * 256.0) as u8,
            (self.y * 256.0) as u8,
            (self.z * 256.0) as u8,
            (self.w * 256.0) as u8,
        ]
    }
}

impl Sub for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Add for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Self) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Mul for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Div for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl Mul<Vector4> for f64 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Vector4 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

