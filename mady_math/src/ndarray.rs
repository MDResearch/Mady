use super::grad::Zero;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

struct NDArray<T, D> {
    phantom: PhantomData<D>,
    data: Vec<T>,
    size: Vec<usize>,
}

struct D1;
struct D2;

impl<T> NDArray<T, D1>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    fn new(data: Vec<T>) -> Self {
        Self {
            phantom: PhantomData,
            size: vec![data.len()],
            data,
        }
    }

    fn dot(self: Self, i: Self) -> T {
        return self
            .data
            .iter()
            .zip(i.data.iter())
            .map(|(&x, &y)| x * y)
            .fold(Zero::zero(), |a, b| a + b);
    }

    fn cross(self: Self, i: Self) -> NDArray<T, D1> {
        // https://zh.wikipedia.org/wiki/%E7%9F%A9%E9%98%B5%E5%BE%AE%E7%A7%AF%E5%88%86
        // https://en.wikipedia.org/wiki/Matrix_calculus

        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size)
        }

        todo!();
    }
}

mod tests {
    use super::NDArray;

    #[test]
    fn d1_dot() {
        let vec_a = NDArray::new(vec![1, 2, 3]);
        let vec_b = NDArray::new(vec![6, 7, 3]);
        let result = vec_a.dot(vec_b);
        assert_eq!(result, 29 as i32);
    }
}
