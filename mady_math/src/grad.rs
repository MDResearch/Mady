//! about the std ops trait

/// return one in a type
///
/// `a * a.one() = a`
/// and
/// `a.one() * a = a`
pub trait One: Sized {
    type O0;
    fn one() -> Self::O0;
}

pub trait Zero: Sized {
    type O0;
    fn zero() -> Self::O0;
}

pub trait Max {
    fn max(self, i: Self) -> Self;
}

pub trait Min {
    fn min(self, i: Self) -> Self;
}

/// just a std method with trait
pub trait Pow {
    fn pow(self, exp: u32) -> Self;
}

/// just a std method with trait
pub trait Powi {
    fn powi(self, exp: i32) -> Self;
}

/// just a std method with trait
pub trait Powf {
    fn powf(self, exp: Self) -> Self;
}

pub trait GradAdd<Rhs = Self> {
    type O0;
    type G0;
    type G1;
    fn grad_add(self, rhs: Rhs) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradSub<Rhs = Self> {
    type O0;
    type G0;
    type G1;
    fn grad_sub(self, rhs: Rhs) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradMul<Rhs = Self> {
    type O0;
    type G0;
    type G1;
    fn grad_mul(self, rhs: Rhs) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradDiv<Rhs = Self> {
    type O0;
    type G0;
    type G1;
    fn grad_div(self, rhs: Rhs) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradMin {
    type O0;
    type G0;
    type G1;
    fn grad_min(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradMax {
    type O0;
    type G0;
    type G1;
    fn grad_max(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1));
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

pub mod impl_one {
    use super::One;
    use crate::impl_trait;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl One for $ty {
                type O0 = Self;
                fn one() -> Self::O0 {
                    1
                }
            }
        };
    }

    impl_trait![parse_i, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

    macro_rules! parse_f {
        ($ty:ident) => {
            impl One for $ty {
                type O0 = Self;
                fn one() -> Self::O0 {
                    1.0
                }
            }
        };
    }

    impl_trait![parse_f, f32, f64];
}

pub mod impl_zero {
    use super::Zero;
    use crate::impl_trait;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl Zero for $ty {
                type O0 = Self;
                fn zero() -> Self::O0 {
                    0
                }
            }
        };
    }

    impl_trait![parse_i, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

    macro_rules! parse_f {
        ($ty:ident) => {
            impl Zero for $ty {
                type O0 = Self;
                fn zero() -> Self::O0 {
                    0.0
                }
            }
        };
    }

    impl_trait![parse_f, f32, f64];
}

mod impl_max {
    use super::{GradMax, One, Zero};
    use crate::impl_trait;
    use std::cmp::Ordering;

    macro_rules! parse_i {
        ($ty:ident) => {
            impl GradMax for $ty {
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_max(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_max(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_min(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_min(self: Self, i: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
            impl GradAdd<Self> for $ty {
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_add(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
            impl GradSub<Self> for $ty {
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_sub(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
            impl GradMul<Self> for $ty {
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_mul(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
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
            impl GradDiv<Self> for $ty {
                type O0 = Self;
                type G0 = Self;
                type G1 = Self;
                fn grad_div(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
                    (self / rhs, (Self::one() / rhs, -self / (rhs * rhs)))
                }
            }
        };
    }

    impl_trait![parse, i8, i16, i32, i64, i128, isize, f32, f64];
}

// impl_trait![
//     Max,
//     fn max(self, i: Self) -> Self {
//         std::cmp::max(self, i)
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
//     Max,
//     fn max(self, i: Self) -> Self {
//         if self > i {
//             return self;
//         }
//         i
//     },
//     f32,
//     f64
// ];

// impl_trait![
//     Min,
//     fn min(self, i: Self) -> Self {
//         std::cmp::min(self, i)
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
//     Min,
//     fn min(self, i: Self) -> Self {
//         if self < i {
//             return self;
//         }
//         i
//     },
//     f32,
//     f64
// ];

// impl_trait![
//     Pow,
//     fn pow(self, exp: u32) -> Self {
//         self.pow(exp)
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
//     Powi,
//     fn powi(self, exp: i32) -> Self {
//         self.powi(exp)
//     },
//     f32,
//     f64
// ];

// impl_trait![
//     Powf,
//     fn powf(self, exp: Self) -> Self {
//         self.powf(exp)
//     },
//     f32,
//     f64
// ];

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

// impl_trait![
//     regex
//     {
//         impl a for [T] {}
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
