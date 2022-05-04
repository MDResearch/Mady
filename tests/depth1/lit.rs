use mady::prelude::*;

#[grad]
fn add(a: f64) -> f64 {
    a + 10_f64
}

#[grad]
fn mix(a: f64, b: f64) -> f64 {
    (a * 6_f64) / (b - 2_f64)
}

fn main() {
    assert_eq!((16., 1.), add(6.));
    assert_eq!((18., (3., -9.)), mix(6., 4.));
}