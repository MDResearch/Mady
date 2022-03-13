use mady::prelude::*;
use rand::prelude::*;

fn main() {
    let mut ann = ANN::new(500, 0.02);
    for _ in 0..100 {
        ann.train()
    }
}

struct ANN {
    layer: Vec<NDArray<D1, f64>>,
    rate: f64,
}

impl ANN {
    pub fn new(size: usize, rate: f64) -> Self {
        let mut layer = vec![];
        let mut rng = thread_rng();
        for _ in 0..size {
            layer.push(NDArray::<D1, _>::new(vec![rng.gen::<f64>()]));
        }
        Self { layer, rate }
    }

    pub fn train(&mut self) {
        let mut rng = thread_rng();
        let mut diff = vec![];
        let mut prev = Array1::new(vec![rng.gen::<f64>() * 360.0]);
        let crr = prev[0].sin();
        for w in self.layer.clone() {
            let (tmp, (_, w_diff)) = grad_layer(prev, w);
            prev = Array1::new(vec![*tmp]);
            diff.push(w_diff);
        }
        println!("error: {}", 2.0 * (prev[0] - crr).powi(2));
        let b = (prev[0] - crr).powi(2);
        for (w, d) in self.layer.iter_mut().zip(diff.into_iter()) {
            w[0] -= self.rate * d[0] * b;
        }
    }
}

#[grad]
fn layer(prev: Array1<f64>, weight: Array1<f64>) -> Array0<f64> {
    prev.dot(weight).relu()
}
