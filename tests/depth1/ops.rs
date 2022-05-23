use mady::prelude::*;

#[grad(f64, f64)]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[grad(f64, f64)]
fn sub(a: f64, b: f64) -> f64 {
    a - b
}

#[grad(f64, f64)]
fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[grad(f64, f64)]
fn div(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    assert_eq!((8., (1., 1.)), add(6., 2.));
    assert_eq!((4., (1., -1.)), sub(6., 2.));
    assert_eq!((12., (2., 6.)), mul(6., 2.));
    assert_eq!((3., (0.5, -1.5)), div(6., 2.));
}
