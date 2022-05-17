use std::ops::{Add, Div, Mul, Sub};

use mady::prelude::Zero;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> From<T> for Complex<T>
where
    T: Zero + Sub<Output = T> + Copy,
{
    fn from(src: T) -> Self {
        Complex {
            real: src,
            imaginary: src - src,
        }
    }
}

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl<T> Sub<Complex<T>> for Complex<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.imaginary * rhs.real + rhs.imaginary * self.real,
        }
    }
}

impl<T> Div<Complex<T>> for Complex<T>
where
    T: Add<T> + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let a = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
        Complex {
            real: (self.real * rhs.real + self.imaginary * rhs.imaginary) / a,
            imaginary: (rhs.real * self.imaginary - self.real * rhs.imaginary) / a,
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = Complex {
            real: 1_i32,
            imaginary: 1_i32,
        };
        let b = Complex {
            real: 2_i32,
            imaginary: 1_i32,
        };
        let c = Complex {
            real: 3_i32,
            imaginary: 2_i32,
        };
        assert_eq!(a + b, c);
    }
    #[test]
    fn sub() {
        let a = Complex {
            real: 8_i32,
            imaginary: 9_i32,
        };
        let b = Complex {
            real: 2_i32,
            imaginary: 1_i32,
        };
        let c = Complex {
            real: 6_i32,
            imaginary: 8_i32,
        };
        assert_eq!(a - b, c);
    }
    #[test]
    fn mul() {
        let a = Complex {
            real: 1_i32,
            imaginary: 1_i32,
        };
        let b = Complex {
            real: 2_i32,
            imaginary: 1_i32,
        };
        let c = Complex {
            real: 1_i32,
            imaginary: 3_i32,
        };
        assert_eq!(a * b, c);
    }
    #[test]
    fn div() {
        let a = Complex {
            real: 9_f64,
            imaginary: 3_f64,
        };
        let b = Complex {
            real: 1_f64,
            imaginary: 2_f64,
        };

        let c = Complex {
            real: 3_f64,
            imaginary: -3_f64,
        };
        assert_eq!(a / b, c);
    }
}
