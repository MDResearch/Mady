use mady::prelude::*;
// use rand::prelude::*;
// use rand_distr::{Distribution, Normal, Standard};
// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     println!("here {}", std::env::current_exe().unwrap().to_str().unwrap());
//     let mut ann = ANN::new(10000);
//     let mut file = File::create("./ann.csv").unwrap();
//     writeln!(file, "epoch,error").unwrap();

//     let mut rng = thread_rng();
//     for i in 0..100 {
//         let output: f64 = rng.gen::<f64>();
//         let input = output.asin();
//         let o = ann.train(Array1::new(vec![input]), Array1::new(vec![output]));
//         writeln!(file, "{},{}", i, o.1).unwrap();
//     }
// }

// // y y-hat -> out, dx
// // w dw
// struct ANN {
//     layer: Vec<Vec<NDArray<D1, f64>>>,
// }

// impl ANN {
//     pub fn new(size: usize) -> Self {
//         let mut layer = vec![];
//         let mut rng = thread_rng();
//         layer.push(vec![]);
//         for _ in 0..size {
//             let config = Normal::new(0.0, (2.0_f64 / 1.0).sqrt()).unwrap();
//             layer
//                 .last_mut()
//                 .unwrap()
//                 .push(Array1::new(vec![config.sample(&mut rng)]));
//         }
//         layer.push(vec![]);
//         let config = Normal::new(0.0, (2.0 / (size as f64)).sqrt()).unwrap();
//         layer.last_mut().unwrap().push(Array1::new(
//             (0..size).map(|_| config.sample(&mut rng)).collect(),
//         ));
//         Self { layer }
//     }

//     pub fn train(&mut self, input: Array1<f64>, output: Array1<f64>) -> (f64, f64) {
//         let mut diff = vec![];
//         let mut prev = input;
//         for l in self.layer.iter() {
//             let mut next = vec![];
//             let mut tmp_diff = vec![];
//             for w in l {
//                 let (tmp, (n_diff, w_diff)) = grad_layer(prev.clone(), w.clone());
//                 tmp_diff.push((n_diff, w_diff));
//                 next.push(*tmp);
//             }
//             prev = Array1::new(next);
//             diff.push(tmp_diff);
//         }
//         let (err, mut dx) = error(output, prev.clone());

//         for (l, w) in diff.into_iter().rev().zip(self.layer.iter_mut().rev()) {
//             let mut tmp = Array1::one();
//             for (((n_diff, w_diff), d), sw) in l.into_iter().zip(dx.into_iter()).zip(w) {
//                 tmp += n_diff.clone() * Array0::new(d);
//                 sgd(sw, w_diff.clone() * Array0::new(d))
//             }
//             dx = tmp;
//         }

//         (prev[0], *err)
//     }
// }

// #[grad]
// fn layer(prev: Array1<f64>, weight: Array1<f64>) -> Array0<f64> {
//     prev.dot(weight).relu()
// }

// fn error(y: Array1<f64>, y_hat: Array1<f64>) -> (Array0<f64>, Array1<f64>) {
//     (
//         Array0::new((y_hat[0] - y[0]).powi(2) / 2.0),
//         Array1::new(vec![y_hat[0] - y[0]]),
//     )
// }

// fn sgd(w: &mut Array1<f64>, d: Array1<f64>) {
//     *w = w.clone() - d * Array0::new(0.0005);
// }

fn main() {
}

// use mady_0::a;
// mod mady_0 {
//     use super::*;

//     type mady_ty_2 = <mady_ty_0 as GradAdd<mady_ty_1>>::O0;
//     type mady_gty_2 = <mady_ty_2 as One>::O0;
//     type mady_ty_2_0 = <mady_ty_0 as GradAdd<mady_ty_1>>::G0;
//     type mady_gty_0 = <mady_gty_2 as MadyChain<mady_ty_2_0>>::O0;
//     type mady_ty_2_1 = <mady_ty_0 as GradAdd<mady_ty_1>>::G1;
//     type mady_gty_1 = <mady_gty_2 as MadyChain<mady_ty_2_1>>::O0;
//     type mady_ty_1 = usize;
//     type mady_ty_0 = usize;
//     pub fn a(a: usize, b: usize) -> (usize, (mady_gty_0, mady_gty_1)) {
//         let mady_tmp_0: mady_gty_0;
//         let mady_tmp_1: mady_gty_1;
//         let mady_tmp_2: mady_gty_2;
//         let mady_tmp_2_0: mady_ty_2_0;
//         let mady_tmp_2_1: mady_ty_2_1;
//         {
//             let mady_return = {
//                 let mady_tmp;
//                 (mady_tmp, (mady_tmp_2_0, mady_tmp_2_1)) = a.grad_add(b);
//                 mady_tmp
//             };
//             mady_tmp_2 = mady_ty_2::one();
//             mady_tmp_0 = mady_tmp_2.mady_chain(mady_tmp_2_0);
//             mady_tmp_1 = mady_tmp_2.mady_chain(mady_tmp_2_1);
//             (mady_return, (mady_tmp_0, mady_tmp_1))
//         }
//     }
// }

use mady_0::a;
mod mady_0 {
    use super::*;
    type mady_ty_2 = <mady_ty_0 as GradAdd<mady_ty_1>>::O0;
    type mady_gty_2 = <mady_ty_2 as One>::O0;
    type mady_ty_2_0 = <mady_ty_0 as GradAdd<mady_ty_1>>::G0;
    type mady_gty_0 = <mady_gty_2 as MadyChain<mady_ty_2_0>>::O0;
    type mady_ty_2_1 = <mady_ty_0 as GradAdd<mady_ty_1>>::G1;
    type mady_gty_1 = <mady_gty_2 as MadyChain<mady_ty_2_1>>::O0;
    type mady_ty_1 = usize;
    type mady_ty_0 = usize;
    pub fn a(a: usize, b: usize) -> (usize, (mady_gty_0, mady_gty_1)) {
        let mady_tmp_0: mady_gty_0;
        let mady_tmp_1: mady_gty_1;
        let mady_tmp_2: mady_gty_2;
        let mady_tmp_2_0: mady_ty_2_0;
        let mady_tmp_2_1: mady_ty_2_1;
        {
            let mady_return = {
                let mady_tmp;
                (mady_tmp, (mady_tmp_2_0, mady_tmp_2_1)) = a.grad_add(b);
                mady_tmp
            };
            mady_tmp_2 = mady_ty_2::one();
            mady_tmp_0 = mady_tmp_2.mady_chain(mady_tmp_2_0);
            mady_tmp_1 = mady_tmp_2.mady_chain(mady_tmp_2_1);
            (mady_return, (mady_tmp_0, mady_tmp_1))
        }
    }
}
