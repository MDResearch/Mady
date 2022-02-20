//! about the std ops trait

use std::error::Error;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::impl_trait;

trait One: Sized {
    fn one() -> Self;
}

trait GradPow: Sized {
    fn grad_pow(self: Self, i: u32) -> (Self, (Self,));
}

trait GradPowi: Sized {
    fn grad_powi(self: Self, i: i32) -> (Self, (Self,));
}

trait GradPowf: Sized {
    fn grad_powf(self: Self, i: Self) -> (Self, (Self,));
}

trait Pow: Sized {
    fn pow(self, exp: u32) -> Self;
}

trait Powi: Sized {
    fn powi(self, exp: i32) -> Self;
}

trait Powf: Sized {
    fn powf(self, exp: Self) -> Self;
}

trait GradAdd<Rhs = Self>
where
    Self: Add<Rhs> + One,
    Rhs: One,
{
    fn grad_add(self, rhs: Rhs) -> (<Self as Add<Rhs>>::Output, (Rhs, Self)) {
        (self + rhs, (Rhs::one(), Self::one()))
    }
}

trait GradSub<Rhs = Self>
where
    Self: Sub<Rhs> + One + Neg,
    Rhs: One,
{
    fn grad_sub(self, rhs: Rhs) -> (<Self as Sub<Rhs>>::Output, (Rhs, <Self as Neg>::Output)) {
        (self - rhs, (Rhs::one(), -Self::one()))
    }
}

trait GradMul<Rhs = Self>
where
    Self: Mul<Rhs> + Clone,
    Rhs: Clone,
{
    fn grad_mul(self, rhs: Rhs) -> (<Self as Mul<Rhs>>::Output, (Rhs, Self)) {
        (self.clone() * rhs.clone(), (rhs, self))
    }
}

trait GradDiv<Rhs = Self>
where
    Self: Div<Rhs> + Neg + Clone,
    Rhs: Div + Div<<Self as Neg>::Output> + Mul + Clone + One,
    <Self as Neg>::Output: Div<<Rhs as Mul>::Output>,
{
    fn grad_div(
        self,
        rhs: Rhs,
    ) -> (
        <Self as Div<Rhs>>::Output,
        (
            <Rhs as Div>::Output,
            <<Self as Neg>::Output as Div<<Rhs as Mul>::Output>>::Output,
        ),
    ) {
        (
            self.clone() / rhs.clone(),
            (Rhs::one() / rhs.clone(), -self / (rhs.clone() * rhs)),
        )
    }
}

// trait GradPow<Rhs = Self>
// where
//     Self: Clone + Pow + Mul<Self, Output = Self> + TryFrom<u32>,
// {
//     fn grad_pow(self, i: u32) -> Result<(Self, (Self,)), Self::Error> {
//         let a: Self = i.clone().try_into()?; // Error when excess range of u32
//         let b = self.clone().pow(i.clone() - 1);
//         let out = a * b;
//         Ok((self.clone(), (out,)))
//     }
// }

// trait GradPowi<Rhs = Self>
// where
//     Self: Clone + Powi + Mul<Self, Output = Self> + TryFrom<i32>,
// {
//     fn grad_powi(self, i: i32) -> Result<(Self, (Self,)), Self::Error> {
//         let a: Self = i.clone().try_into()?;
//         let b = self.clone().powi(i.clone() - 1);
//         let out = a * b;
//         Ok((self.clone(), (out,)))
//     }
// }

// trait GradPowf<Rhs = Self>
// where
//     Self: Clone + Powf + Mul<Self, Output = Self> + Sub<Self, Output = Self> + TryFrom<usize>,
// {
//     fn grad_powf(self, i: Self) -> Result<(Self, (Self,)), Self::Error> {
//         let one = 1usize.try_into()?;
//         let b = self.clone().powf(i.clone() - one);
//         let out = i * b;
//         Ok((self.clone(), (out,)))
//     }
// }

// impl traits
impl_trait![
    One,
    fn one() -> Self {
        1
    },
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
];

impl_trait![
    One,
    fn one() -> Self {
        1.0
    },
    f32,
    f64
];


impl_trait![
    Pow,
    fn pow(self, exp: u32) -> Self {
        self.pow(exp)
    },
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
];

impl_trait![
    Powi,
    fn powi(self, exp: i32) -> Self {
        self.powi(exp)
    },
    f32,
    f64
];


impl_trait![
    GradPow,
    fn grad_pow(self, i: u32) -> (Self, (Self,)) {
        let a: Self = i.clone() as Self;
        let b = self.clone().pow(i.clone() - 1);
        let out = a * b;
        (self.clone(), (out,))
    },
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
];

impl_trait![
    GradPowi,
    fn grad_powi(self, i: i32) -> (Self, (Self,)) {
        let a: Self = i.clone() as Self;
        let b = self.clone().powi(i.clone() - 1);
        let out = a * b;
        (self.clone(), (out,))
    },
    f32,
    f64
];

impl_trait![
    GradPowf,
    fn grad_powf(self, i: Self) -> (Self, (Self,)) {
        let a: Self = i.clone() as Self;
        let b = self.clone().powf(i.clone() - 1 as Self);
        let out = a * b;
        (self.clone(), (out,))
    },
    f32,
    f64
];

impl_trait![
    Powf,
    fn powf(self, exp: Self) -> Self {
        self.powf(exp)
    },
    f32,
    f64
];


impl_trait![GradAdd, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait![GradSub, i8, i16, i32, i64, i128, f32, f64];

impl_trait![GradMul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait![GradDiv, i8, i16, i32, i64, i128, f32, f64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grad_add() {
        let a = 1;
        let b = 2;

        assert_eq!(a.grad_add(b), (3, (1, 1)));
    }

    #[test]
    fn grad_sub() {
        let a = 1;
        let b = 2;

        assert_eq!(a.grad_sub(b), (-1, (1, -1)));
    }

    #[test]
    fn grad_mul() {
        let a = 1;
        let b = 2;

        assert_eq!(a.grad_mul(b), (2, (2, 1)));
    }

    #[test]
    fn grad_div() {
        let a = 4.0;
        let b = 2.0;

        assert_eq!(a.grad_div(b), (2.0, (1.0 / 2.0, -4.0 / 2.0 / 2.0)));
    }
}
