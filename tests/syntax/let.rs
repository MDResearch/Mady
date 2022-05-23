use mady::prelude::*;

#[grad(f64)]
fn let_mul(a: f64) -> f64 {
    let mut b = a;
    b = b * a;
    b = b * a;
    b = b * a;
    b
}

#[grad(f64, f64)]
fn classic(a: f64, b: f64) -> f64 {
    let c = (a + b) * a;
    c
}

#[grad(f64, f64)]
fn separate(a: f64, b: f64) -> f64 {
    let c;
    c = (a + b) * a;
    c
}

#[grad(f64, f64)]
fn separate_multiple(a: f64, b: f64) -> f64 {
    let c = (a + b) * a;
    let d = c;
    let e = d;
    let f;
    let g;
    f = e;
    g = f;
    g
}

fn main() {
    assert_eq!((16., 32.), let_mul(2.));
    assert_eq!((48., (14., 6.)), classic(6., 2.));
    assert_eq!((48., (14., 6.)), separate(6., 2.));
    assert_eq!((48., (14., 6.)), separate_multiple(6., 2.));
}
