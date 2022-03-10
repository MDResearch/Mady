use super::grad::Zero;
use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug)]
struct NDArray<T, D> {
    phantom: PhantomData<D>,
    data: Vec<T>,
    size: Vec<usize>,
}

#[derive(Debug)]
struct D1;

#[derive(Debug)]
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
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        self.data
            .iter()
            .zip(i.data.iter())
            .map(|(&x, &y)| x * y)
            .fold(Zero::zero(), |a, b| a + b)
    }

    fn cross(self: Self, i: Self) -> NDArray<T, D1> {
        // https://zh.wikipedia.org/wiki/%E7%9F%A9%E9%98%B5%E5%BE%AE%E7%A7%AF%E5%88%86
        // https://en.wikipedia.org/wiki/Matrix_calculus

        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        todo!();
    }
}

impl<T> NDArray<T, D2>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    fn new(data: Vec<T>, size: (usize, usize)) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(size.0 * size.1, data.len());
        }
        Self {
            phantom: PhantomData,
            size: vec![size.0, size.1],
            data,
        }
    }

    fn mul(self: Self, i: NDArray<T, D1>) -> NDArray<T, D1> {
        let mut result = vec![];
        if cfg!(debug_assertions) {
            assert_eq!(self.size[0], i.size[0]);
        }
        for j in 0..self.size[1] {
            let mut sum = Zero::zero();
            for c in 0..self.size[0] {
                sum = sum + self.data[j * self.size[0] + c] * i.data[c];
            }
            result.push(sum);
        }

        NDArray::<T, D1>::new(result)
    }

    fn grad_mul(
        self: Self,
        i: NDArray<T, D1>,
    ) -> (NDArray<T, D1>, (NDArray<T, D2>, NDArray<T, D1>)) {
        let mut result = vec![];
        if cfg!(debug_assertions) {
            assert_eq!(self.size[0], i.size[0]);
        }
        for j in 0..self.size[1] {
            let mut sum: T = Zero::zero();
            for c in 0..self.size[0] {
                sum = sum + self.data[j * self.size[0] + c] * i.data[c];
            }
            result.push(sum);
        }

        NDArray::<T, D1>::new(result);
        todo!()
    }
}

mod tests {
    use super::{NDArray, D1, D2};

    #[test]
    fn d1_dot() {
        let vec_a = NDArray::<i32, D1>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<i32, D1>::new(vec![6, 7, 3]);
        let result = vec_a.dot(vec_b);

        assert_eq!(result, 29 as i32);
    }

    #[test]
    fn d2_mul_d1() {
        let vec_a = NDArray::<i32, D2>::new(vec![1, 2, 3, 4], (2, 2));
        let vec_b = NDArray::<i32, D1>::new(vec![7, 1]);
        let result = vec_a.mul(vec_b);

        assert_eq!(result.data[0], 9);
        assert_eq!(result.data[1], 25);
    }
}
