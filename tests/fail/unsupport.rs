use mady::prelude::*;

#[grad]
fn unsupport_ops(a: f64, b: f64) -> f64 {
   a >> b ^ b << a
}

#[grad]
fn cannot_infer_type(a: f64, b: f64) -> f64 {
    a * b + 10
}

fn main() {
}