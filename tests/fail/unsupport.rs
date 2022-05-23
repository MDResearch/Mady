use mady::prelude::*;

#[grad(f64, f64)]
fn unsupport_ops(a: f64, b: f64) -> f64 {
    a >> b ^ b << a
}

fn main() {}
