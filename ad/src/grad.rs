use std::ops::{Add, Mul, Neg, Sub};

use crate::{impl_const_trait, impl_trait_default};

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
    Self: Mul<Rhs> + Clone,
    Rhs: Clone,
{
    fn grad_div(self, rhs: Rhs) -> (<Self as Mul<Rhs>>::Output, (Rhs, Self)) {
        (self.clone() * rhs.clone(), (rhs, self))
    }
}

impl_const_trait![One, one, 1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradAdd, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradSub, i8, i16, i32, i64, i128, f32, f64];

impl_trait_default![GradMul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grad_add() {
        let a = 1;
        let b = 2;

        assert_eq!(a.grad_add(b), (3, (1, 1)));
    }
}
