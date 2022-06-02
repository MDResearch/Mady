use std::ops::{Add, Div, Mul, Sub};

use mady::prelude::*;

/// definition of complex number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T>
where
    Self: MadyZero + Copy,
{
    pub fn new(real: T, imaginary: T) -> Self {
        Self { real, imaginary }
    }

    pub fn grad_new(real: T, imaginary: T) -> (Self, (Self, Self)) {
        (
            Self::new(real, imaginary),
            (Self::mady_zero(), Self::mady_zero()),
        )
    }
}

impl<T> MadyChain for Complex<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mady_chain(self, i: Self) -> Self::Output {
        self * i
    }
}

impl<T> From<T> for Complex<T>
where
    T: MadyZero + Copy,
{
    fn from(src: T) -> Self {
        Complex {
            real: src,
            imaginary: T::mady_zero(),
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

impl<T> MadyOne for Complex<T>
where
    T: MadyOne<Output = T> + MadyZero + Copy,
{
    type Output = Complex<T>;
    fn mady_one() -> Self::Output {
        Complex {
            real: T::mady_one(),
            imaginary: T::mady_zero(),
        }
    }
}

impl<T> MadyZero for Complex<T>
where
    T: MadyZero + Copy,
{
    fn mady_zero() -> Self {
        Complex {
            real: T::mady_zero(),
            imaginary: T::mady_zero(),
        }
    }
}

/// add custom differential function for Complex
///
/// adding #[marco@grad] might work in the future version.
impl<T> GradAdd<Complex<T>> for Complex<T>
where
    T: MadyOne<Output = T> + Copy,
    Complex<T>: Add<Output = Complex<T>> + Copy + MadyOne<Output = Complex<T>>,
{
    type Output = Complex<T>;
    type GradLeft = Complex<T>;
    type GradRight = Complex<T>;

    fn grad_add(self, rhs: Complex<T>) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
        (self + rhs, (Complex::mady_one(), Complex::mady_one()))
    }
}

impl<T> GradSub<Complex<T>> for Complex<T>
where
    Complex<T>: MadyZero + Sub<Output = Complex<T>> + Copy + MadyOne<Output = Complex<T>>,
{
    type Output = Complex<T>;
    type GradLeft = Complex<T>;
    type GradRight = Complex<T>;

    fn grad_sub(self, rhs: Complex<T>) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
        (
            self - rhs,
            (
                Complex::mady_one(),
                Complex::mady_zero() - Complex::mady_one(),
            ),
        )
    }
}

impl<T> GradMul<Complex<T>> for Complex<T>
where
    Complex<T>: Mul<Output = Complex<T>> + Copy + MadyOne<Output = Complex<T>>,
{
    type Output = Complex<T>;
    type GradLeft = Complex<T>;
    type GradRight = Complex<T>;

    fn grad_mul(self, rhs: Complex<T>) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
        (self * rhs, (rhs, self))
    }
}

/// gradrotate_37
#[grad(Complex<f64>)]
fn rotate_37(a: Complex<f64>) -> Complex<f64> {
    a * Complex::new(0.8, 0.6)
}

fn main() {
    /// a's partial derivative to rotate_37() is 0.8+0.6i, "i" is the square root of -1
    assert_eq!(rotate_37(Complex::new(3., 4.)).1, Complex::new(0.8, 0.6));
}

impl GradSin for f64 {
    fn gradsin(self) -> self {
        self.cos()
    }
}

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
