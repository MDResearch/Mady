use mady::prelude::*;

#[grad]
fn pow_6(a: f64) -> f64 {
    a * a * a * a * a * a
}

#[grad]
fn classic(a: f64, b: f64) -> f64 {
    (a + b) * a
}

#[grad]
fn input_3(a: f64, b: f64, c: f64) -> f64 {
    (a * b + b * c + c * a) * (a + b + c)
}

#[grad]
fn a_mul_b_6(a: f64, b: f64) -> f64 {
    a * b * a * b * a * b
}


fn main() {
    assert_eq!((64., 192.), pow_6(2.));
    assert_eq!((48., (14., 6.)), classic(6., 2.));
    assert_eq!((396., (91., 135., 124.)), input_3(6., 2. ,3.));
    assert_eq!((1728., (864., 2592.)), a_mul_b_6(6., 2.));
}