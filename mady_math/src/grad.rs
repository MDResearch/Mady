//! about the std ops trait

pub struct MadyNull;

pub trait MadyChain<Rhs = Self> {
    type Output;
    fn mady_chain(self, i: Rhs) -> Self::Output;
}

/// return one in a type
///
/// one must can
/// `a * a.one() = a`
/// and
/// `a.one() * a = a`
pub trait One: Sized {
    type Output;
    fn one() -> Self::Output;
}

/// return zero in a type
///
/// zero mean it not affact function output
pub trait Zero: Sized {
    fn zero() -> Self;
}

pub trait GradAdd<Rhs = Self> {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_add(self, rhs: Rhs) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

pub trait GradSub<Rhs = Self> {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_sub(self, rhs: Rhs) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

pub trait GradMul<Rhs = Self> {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_mul(self, rhs: Rhs) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

pub trait GradDiv<Rhs = Self> {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_div(self, rhs: Rhs) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

pub trait GradMin {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_min(self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

pub trait GradMax {
    type Output;
    type GradLeft;
    type GradRight;
    fn grad_max(self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight));
}

// pub trait GradPow {
//     fn grad_pow(self: Self, i: u32) -> (Self, (Self,));
// }

// pub trait GradPowi {
//     fn grad_powi(self: Self, i: i32) -> (Self, (Self,));
// }

// pub trait GradPowf {
//     fn grad_powf(self: Self, i: Self) -> (Self, (Self,));
// }

// pub trait Gradtanh {
//     fn grad_tanh(i: Self) -> (Self, (Self,));
// }

mod impl_chain {
    use super::MadyChain;
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl MadyChain for $ty {
                type Output = Self;
                fn mady_chain(self, rhs: Self) -> Self::Output {
                    self * rhs
                }
            }
        };
    }

    impl_trait![parse, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];
}

mod impl_one {
    use super::{MadyNull, One};
    use crate::impl_trait;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl One for $ty {
                type Output = Self;
                fn one() -> Self::Output {
                    1
                }
            }
        };
    }

    impl_trait![parse_i, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

    macro_rules! parse_f {
        ($ty:ident) => {
            impl One for $ty {
                type Output = Self;
                fn one() -> Self::Output {
                    1.
                }
            }
        };
    }

    impl_trait![parse_f, f32, f64];

    macro_rules! parse_p {
        ($ty:ident) => {
            impl One for $ty {
                type Output = Self;
                fn one() -> Self::Output {
                    Self
                }
            }
        };
    }

    impl_trait![parse_p, MadyNull];
}

mod impl_zero {
    use super::Zero;
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl Zero for $ty {
                fn zero() -> Self {
                    Self::default()
                }
            }
        };
    }

    impl_trait![parse, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];
}

mod impl_max {
    use super::{GradMax, One, Zero};
    use crate::impl_trait;
    use std::cmp::Ordering;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl GradMax for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_max(self: Self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    match self.cmp(&i) {
                        Ordering::Less | Ordering::Equal => (i, (Self::zero(), Self::one())),
                        Ordering::Greater => (self, (Self::one(), Self::zero())),
                    }
                }
            }
        };
    }

    impl_trait![parse_i, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

    macro_rules! parse_f {
        ($ty:ident) => {
            impl GradMax for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_max(self: Self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    match self.partial_cmp(&i).unwrap() {
                        Ordering::Less | Ordering::Equal => (i, (Self::zero(), Self::one())),
                        Ordering::Greater => (self, (Self::one(), Self::zero())),
                    }
                }
            }
        };
    }

    impl_trait![parse_f, f32, f64];
}

mod impl_min {
    use super::{GradMin, One, Zero};
    use crate::impl_trait;
    use std::cmp::Ordering;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl GradMin for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_min(self: Self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    match self.partial_cmp(&i).unwrap() {
                        Ordering::Less | Ordering::Equal => (self, (Self::one(), Self::zero())),
                        Ordering::Greater => (i, (Self::zero(), Self::one())),
                    }
                }
            }
        };
    }

    impl_trait![parse_i, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

    macro_rules! parse_f {
        ($ty:ident) => {
            impl GradMin for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_min(self: Self, i: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    match self.partial_cmp(&i).unwrap() {
                        Ordering::Less | Ordering::Equal => (self, (Self::one(), Self::zero())),
                        Ordering::Greater => (i, (Self::zero(), Self::one())),
                    }
                }
            }
        };
    }
    impl_trait![parse_f, f32, f64];
}

mod impl_add {
    use super::{GradAdd, One};
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl GradAdd for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_add(self, rhs: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    (self + rhs, (Self::one(), Self::one()))
                }
            }
        };
    }

    impl_trait![parse, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];
}

mod impl_sub {
    use super::{GradSub, One};
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl GradSub for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_sub(self, rhs: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    (self - rhs, (Self::one(), -Self::one()))
                }
            }
        };
    }

    impl_trait![parse, i8, i16, i32, i64, i128, isize, f32, f64];
}

mod impl_mul {
    use super::GradMul;
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl GradMul for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_mul(self, rhs: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    (self * rhs, (rhs, self))
                }
            }
        };
    }

    impl_trait![parse, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];
}

mod impl_div {
    use super::{GradDiv, One};
    use crate::impl_trait;

    macro_rules! parse {
        ($ty:ident) => {
            impl GradDiv for $ty {
                type Output = Self;
                type GradLeft = Self;
                type GradRight = Self;
                fn grad_div(self, rhs: Self) -> (Self::Output, (Self::GradLeft, Self::GradRight)) {
                    (self / rhs, (Self::one() / rhs, -self / (rhs * rhs)))
                }
            }
        };
    }

    impl_trait![parse, i8, i16, i32, i64, i128, isize, f32, f64];
}


// impl_trait![
//     GradPow,
//     fn grad_pow(self, i: u32) -> (Self, (Self,)) {
//         let a: Self = i as Self;
//         let b = self.pow(i.clone() - 1);
//         let out = a * b;
//         (self.pow(i), (out,))
//     },
//     u8,
//     u16,
//     u32,
//     u64,
//     u128,
//     usize,
//     i8,
//     i16,
//     i32,
//     i64,
//     i128,
//     isize
// ];

// impl_trait![
//     Gradtanh,
//     fn grad_tanh(i: Self) -> (Self, (Self,)) {
//         // be warning that it seems to be wrong
//         (i.tanh(), (i.atanh().acosh().powf(-2.0),))
//     },
//     f32,
//     f64
// ];

// impl_trait![
//     GradPowi,
//     fn grad_powi(self, i: i32) -> (Self, (Self,)) {
//         let a: Self = i.clone() as Self;
//         let b = self.clone().powi(i.clone() - 1);
//         let out = a * b;
//         (self.powi(i), (out,))
//     },
//     f32,
//     f64
// ];

// impl_trait![
//     GradPowf,
//     fn grad_powf(self, i: Self) -> (Self, (Self,)) {
//         let a: Self = i.clone() as Self;
//         let b = self.powf(i.clone() - 1.0);
//         let out = a * b;
//         (self.powf(i), (out,))
//     },
//     f32,
//     f64
// ];


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

    // !low
    // #[test]
    // fn grad_pow() {
    //     let a = 4;
    //     let b = 2;

    //     assert_eq!(a.grad_pow(b), (4.pow(2), (4.pow(2 - 1) * 2,)));
    // }

    #[test]
    fn grad_min() {
        let a = 8.0;
        let b = 20.0;

        assert_eq!(a.grad_min(b), (a, (1.0, 0.0)));
    }

    #[test]
    fn grad_max() {
        let a = 8.0;
        let b = 20.0;

        assert_eq!(a.grad_max(b), (b, (0.0, 1.0)));
    }
}
