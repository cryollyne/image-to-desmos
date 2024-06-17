use std::ops::{Add, Sub, Mul};

use crate::utils::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    pub fn from_real(real: f64) -> Complex {
        Complex { real, imaginary: 0.0 }
    }

    pub fn from_cartisian(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }

    pub fn from_polar(magnitude: f64, angle: f64) -> Complex {
        Complex {
            real: magnitude * f64::cos(angle),
            imaginary: magnitude * f64::sin(angle),
        }
    }

    pub fn from_vector2(v: Vector2) -> Complex {
        Complex{
            real: v.x,
            imaginary: v.y,
        }
    }

    pub fn pow(self, exp: f64) -> Complex {
        let angle = f64::atan2(self.imaginary, self.real);
        let magnitude = f64::sqrt(self.real*self.real + self.imaginary*self.imaginary);
        Complex::from_polar(magnitude.powf(exp), exp * angle)
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real*rhs.real - self.imaginary*rhs.imaginary,
            imaginary: self.real*rhs.imaginary + self.imaginary*rhs.real,
        }
    }
}
