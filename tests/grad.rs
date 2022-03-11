use mady::prelude::*;

// #[grad]
// fn ops_add(a: usize, b: usize) -> usize {
//     a + b
// }

// #[grad(usize, usize)]
// fn ops_mul(a: usize, b: usize) -> usize {
//     a * b
// }

// #[test]
// fn test_ops() {
//     assert_eq!(grad_ops_add(6, 23), (29, (1, 1)));
//     assert_eq!(grad_ops_mul(6, 23), (138, (23, 6)));
// }

// fn tt()  {
//     let a=usize::zero();
// }

// #[grad(usize, usize)]
// fn ops_mul_add(a: usize, b: usize) -> usize {
//     a * b + a
// }

// fn grad(a: usize, b: usize) -> (usize, (usize, usize)) {
// let a:usize=Zero::zero();
// let mut mady_0 = Zero::zero();
// let mut mady_1 = Zero::zero();
// let mady_2: usize = One::one();
// // this
// let mut mady_3: usize;
// let mut mady_4 = Zero::zero();
// let mut mady_5 = Zero::zero();
// // this
// let mut mady_6: usize;
// let c = {
//     mady_6 = One::one();
//     {
//         let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_add(b);
//         mady_4 = mady_tmp1;
//         mady_5 = mady_tmp2;
//         mady_tmp0
//     }
// };
// let mady_return = c;
// mady_3 = mady_2.clone() * mady_6;
// mady_0 += mady_3.clone() * mady_4;
// mady_1 += mady_3.clone() * mady_5;
// (mady_return, (mady_0, mady_1))
// }

fn a_plus_b(a: usize, b: usize) -> usize {
    let c = a + b;
    c
}
fn grad_a_plus_b(a: usize, b: usize) -> (usize, (usize, usize)) {
    let mut mady_0 = usize::zero();
    let mut mady_1 = usize::zero();
    let mady_2 = <usize as GradAdd<usize>>::O0::one();
    let mut mady_3 = <usize as GradAdd<usize>>::G0::zero();
    let mut mady_4 = <usize as GradAdd<usize>>::G1::zero();
    let c = {
        let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_add(b);
        mady_3 = mady_tmp1;
        mady_4 = mady_tmp2;
        mady_tmp0
    };
    let mady_return = c;
    mady_0 += mady_2.clone() * mady_3;
    mady_1 += mady_2.clone() * mady_4;
    (mady_return, (mady_0, mady_1))
}
