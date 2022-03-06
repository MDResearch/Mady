use mady::prelude::*;

#[grad(usize, usize)]
fn ops_add(a: usize, b: usize) -> usize {
    a + b
}

#[grad(usize, usize)]
fn ops_mul(a: usize, b: usize) -> usize {
    a * b
}

#[test]
fn test_ops() {
    assert_eq!(grad_ops_add(6, 23), (29, (1, 1)));
    assert_eq!(grad_ops_mul(6, 23), (138, (23, 6)));
}

// #[grad(usize, usize)]
// fn ops_mul_add(a: usize, b: usize) -> usize {
//     a * b + a
// }

// fn grad(a: usize, b: usize) -> (usize, (usize, usize)) {
//     let mut mady_0 = Zero::zero();
//     let mut mady_1 = Zero::zero();
//     let mady_2: usize = One::one();
//     // this
//     let mut mady_3: need_type = Zero::zero();
//     let mut mady_4 = Zero::zero();
//     let mut mady_5 = Zero::zero();
//     // this
//     let mut mady_6: need_type = Zero::zero();
//     let c = {
//         mady_6 = One::one();
//         {
//             let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_add(b);
//             mady_4 = mady_tmp1;
//             mady_5 = mady_tmp2;
//             mady_tmp0
//         }
//     };
//     let mady_return = c;
//     mady_3 += mady_2.clone() * mady_6;
//     mady_0 += mady_3.clone() * mady_4;
//     mady_1 += mady_3.clone() * mady_5;
//     (mady_return, (mady_0, mady_1))
// }
