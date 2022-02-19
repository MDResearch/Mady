//! about the std ops trait

use std::ops::{Add, Div, Mul, Neg, Sub};

// use std::i32::pow;

use crate::{impl_const_trait, impl_trait_default, impl_trait_self2};

trait One: Sized {
    fn one() -> Self;
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

trait Pow: Sized {
    fn one() -> Self;
}

// trait GradPow<Rhs = Self>
// where
//     Self: Clone + Pow,
//     i32: Clone,
// {
//     fn grad_pow(self, i: u32) {
//         self.pow(i - 1) * i
//     }
// }

// impl One trait
impl_const_trait![One, one, 1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128];
impl_const_trait![One, one, 1.0, f32, f64];

// impl_trait_default![GradPow, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradAdd, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradSub, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradMul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradDiv, i8, i16, i32, i64, i128, f32, f64];

// impl_trait_self2![Pow, pow, [i, i32], self.pow(i), i32];

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
