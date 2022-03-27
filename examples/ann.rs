// use mady::prelude::*;
// use rand::prelude::*;
// use rand_distr::{Distribution, Normal, Standard};
// use std::borrow::BorrowMut;
// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     let mut m = Momentum::new(Array0::new(0.9), Array0::new(0.01));
//     let mut ann = ANN::new(&[1, 6, 10, 6, 1], error, m);
//     let mut file = File::create("ann.csv").unwrap();
//     writeln!(file, "epoch,error").unwrap();

//     let mut rng = thread_rng();
//     for i in 0..800 {
//         let input: f64 = rng.gen::<f64>();
//         let output = input.powi(2);
//         let o = ann.train(Array1::new(vec![input]), Array1::new(vec![output]));
//         writeln!(file, "{},{}", i, *o).unwrap();
//     }
// }

// trait SGD {
//     fn sgd(&mut self, w: &mut Array1<f64>, d: Array1<f64>);
// }

// // y y-hat -> out, dx
// // w dw
// struct ANN<L, S>
// where
//     L: Fn(Array1<f64>, Array1<f64>) -> (Array0<f64>, Array1<f64>),
//     S: SGD,
// {
//     layer: Vec<Vec<NDArray<D1, f64>>>,
//     loss: L,
//     sgd: S,
// }

// impl<L, S> ANN<L, S>
// where
//     L: Fn(Array1<f64>, Array1<f64>) -> (Array0<f64>, Array1<f64>),
//     S: SGD,
// {
//     pub fn new(size: &[usize], loss: L, sgd: S) -> Self {
//         let mut layer = vec![];
//         let mut rng = thread_rng();
//         let mut b = size[0];
//         for i in size.into_iter().skip(1) {
//             let config = Normal::new(0.0, 2.0 / (*i as f64)).unwrap();
//             let mut tmp = vec![];
//             for _ in 0..*i {
//                 tmp.push(Array1::<f64>::new(
//                     (0..b).map(|_| config.sample(&mut rng)).collect(),
//                 ));
//             }
//             layer.push(tmp);
//             b = *i;
//         }
//         Self { layer, loss, sgd }
//     }

//     pub fn train(&mut self, input: Array1<f64>, output: Array1<f64>) -> Array0<f64> {
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
//         let (err, mut dx) = (self.loss)(output, prev);

//         for (l, w) in diff.into_iter().rev().zip(self.layer.iter_mut().rev()) {
//             let mut tmp = Array1::one();
//             for (((n_diff, w_diff), d), sw) in l.into_iter().zip(dx.into_iter()).zip(w) {
//                 tmp += n_diff.clone() * Array0::new(d);
//                 self.sgd.sgd(sw, w_diff.clone() * Array0::new(d));
//             }
//             dx = tmp;
//         }

//         err
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
//     *w = w.clone() - d * Array0::new(0.01);
// }

// struct Momentum {
//     v: Array1<f64>,
//     b: Array0<f64>,
//     n: Array0<f64>,
// }

// impl Momentum {
//     fn new(b: Array0<f64>, n: Array0<f64>) -> Self {
//         Self {
//             v: Array1::<f64>::one(),
//             b,
//             n,
//         }
//     }
// }

// impl SGD for Momentum {
//     fn sgd(&mut self, w: &mut Array1<f64>, d: Array1<f64>) {
//         let t = self.v.clone() * self.b.clone();
//         let k = d * self.n.clone();
//         self.v = t - k;
//         *w = w.clone() + self.v.clone();
//     }
// }

fn main(){
    
}