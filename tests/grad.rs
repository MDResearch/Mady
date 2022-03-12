use mady::prelude::*;

#[grad]
fn ops_add(a: usize, b: usize) -> usize {
    a + b
}

#[grad]
fn ops_mul(a: usize, b: usize) -> usize {
    a * b
}

#[grad]
fn ops_add_mul(a: usize, b: usize) -> usize {
    (a + b) * a
}

#[grad]
fn mult_mul(a: usize) -> usize {
    a * a * a * a * a * a
}

#[grad]
fn with_let(a: usize, b: usize) -> usize {
    a * b
}

#[grad]
fn with_method(a: usize, b: usize) -> usize {
    a.mul(b)
}

#[test]
fn test_ops() {
    assert_eq!(grad_ops_add(6, 23), (ops_add(6, 23), (1, 1)));
    assert_eq!(grad_ops_mul(6, 23), (ops_mul(6, 23), (23, 6)));
}

#[test]
fn test_mult() {
    assert_eq!(grad_mult_mul(2), (mult_mul(2), (6 * 2_usize.pow(5),)));
}

#[test]
fn test_systex() {
    assert_eq!(grad_with_let(6, 23), (with_let(6, 23), (23, 6)));
    assert_eq!(grad_with_method(6, 23), (with_method(6, 23), (23, 6)));
}
