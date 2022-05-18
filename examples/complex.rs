use std::ops::{Add, Div, Mul, Sub};

use mady::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> From<T> for Complex<T>
where
    T: Zero<O0 = T> + Copy,
{
    fn from(src: T) -> Self {
        Complex {
            real: src,
            imaginary: T::zero(),
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

impl<T> One for Complex<T>
where
    T: One<O0 = T> + Zero<O0 = T> + Copy,
{
    type O0 = Complex<T>;
    fn one() -> Self::O0 {
        Complex {
            real: T::one(),
            imaginary: T::zero(),
        }
    }
}

impl<T> Zero for Complex<T>
where
    T: One<O0 = T> + Zero<O0 = T> + Copy,
{
    type O0 = Complex<T>;
    fn zero() -> Self::O0 {
        Complex {
            real: T::zero(),
            imaginary: T::zero(),
        }
    }
}

impl<T> GradAdd<Complex<T>> for Complex<T>
where
    T: One<O0 = T> + Zero<O0 = T> + Copy,
    Complex<T>: Add<Output = Complex<T>> + Copy + One<O0 = Complex<T>>,
{
    type O0 = Complex<T>;
    type G0 = Complex<T>;
    type G1 = Complex<T>;

    fn grad_add(self, rhs: Complex<T>) -> (Self::O0, (Self::G0, Self::G1)) {
        (self + rhs, (Complex::one(), Complex::one()))
    }
}

impl<T> GradSub<Complex<T>> for Complex<T>
where
    T: One<O0 = T> + Zero<O0 = T> + Copy,
    Complex<T>: Sub<Output = Complex<T>> + Copy + One<O0 = Complex<T>>,
{
    type O0 = Complex<T>;
    type G0 = Complex<T>;
    type G1 = Complex<T>;

    fn grad_sub(self, rhs: Complex<T>) -> (Self::O0, (Self::G0, Self::G1)) {
        (
            self - rhs,
            (Complex::one(), Complex::zero() - Complex::one()),
        )
    }
}

impl<T> GradMul<Complex<T>> for Complex<T>
where
    T: One<O0 = T> + Zero<O0 = T> + Copy,
    Complex<T>: Mul<Output = Complex<T>> + Copy + One<O0 = Complex<T>>,
{
    type O0 = Complex<T>;
    type G0 = Complex<T>;
    type G1 = Complex<T>;

    fn grad_mul(self, rhs: Complex<T>) -> (Self::O0, (Self::G0, Self::G1)) {
        (self * rhs, (rhs, self))
    }
}

// #[grad]
fn rotate_37(a: Complex<f64>) -> Complex<f64> {
    todo!();
    a * Complex {
        real: 0.6_f64,
        imaginary: 0.8_f64,
    }
}

// use mady_4722293072650129776::rotate_37;
// mod mady_4722293072650129776 {
//     use super::*;
//     type mady_ty_3 = <mady_ty_1 as GradMul<mady_ty_2>>::O0;
//     type mady_gty_3 = <mady_ty_3 as One>::O0;
//     type mady_ty_4 = <mady_ty_1 as GradMul<mady_ty_2>>::G0;
//     type mady_gty_1 = <mady_gty_3 as MadyChain<mady_ty_4>>::O0;
//     type mady_ty_5 = <mady_ty_1 as GradMul<mady_ty_2>>::G1;
//     type mady_gty_2 = <mady_gty_3 as MadyChain<mady_ty_5>>::O0;
//     type mady_ty_2 = f64;
//     type mady_ty_1 = f64;
//     type mady_ty_0 = Complex<f64>;
//     type mady_gty_0 = <mady_ty_0 as One>::O0;
//     pub fn rotate_37(a: Complex<f64>) -> (Complex<f64>, (mady_gty_0)) {
//         let mut mady_var_0: mady_gty_0;
//         let mut mady_var_1: mady_gty_1;
//         let mut mady_var_2: mady_gty_2;
//         let mut mady_var_3: mady_gty_3;
//         let mady_var_4: mady_ty_4;
//         let mady_var_5: mady_ty_5;
//         let mady_return = {
//             {
//                 {
//                     let mady_tmp;
//                     (mady_tmp, (mady_var_4, mady_var_5)) = a.grad_mul(Complex {
//                         real: 0.6_f64,
//                         imaginary: 0.8_f64,
//                     });
//                     mady_tmp
//                 }
//             }
//         };
//         mady_var_3 = mady_ty_3::one();
//         mady_var_1 = mady_var_3.mady_chain(mady_var_4).clone();
//         mady_var_2 = mady_var_3.mady_chain(mady_var_5).clone();
//         mady_var_0 = mady_ty_0::one();
//         (mady_return, (mady_var_0))
//     }
// }

fn main() {
    let mut x = 50;
    let mut y = 10;
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
