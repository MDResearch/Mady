use mady::prelude::*;

#[grad(f64, f64)]
fn mul(a: f64, b: f64) -> f64 {
    a.mul(b)
}

fn main() {
    assert_eq!((12., (2., 6.)), mul(6., 2.));
}
