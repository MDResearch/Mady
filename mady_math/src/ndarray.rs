use super::grad::Zero;

use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
    process::Output,
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
    T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero<O0 = T> + Copy,
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
            .fold(T::zero(), |a, b| a + b)
    }

    fn mul(self: Self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let mut result = vec![];
        for c in 0..self.data.len() {
            result.push(self.data[c] * i.data[c]);
        }

        NDArray::<T, D1>::new(result)
    }

    // wrong one
    // fn jacobian(self: Self, i: NDArray<T, D1>) -> NDArray<T, D2> {
    //     let mut result = vec![];
    //     for j in 0..self.size[0] {
    //         for c in 0..self.size[0] {
    //             result.push(self.data[c] / i.data[j]);
    //         }
    //     }

    //     NDArray::<T, D2>::new(result, (self.size[0], self.size[0]))
    // }

    fn add(self: Self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let mut result = vec![];
        for c in 0..self.data.len() {
            result.push(self.data[c] + i.data[c]);
        }

        NDArray::<T, D1>::new(result)
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
    T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero<O0 = T> + Copy,
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

    fn add(self: Self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let mut result = vec![];
        for c in 0..self.data.len() {
            result.push(self.data[c] + i.data[c]);
        }

        NDArray::<T, D2>::new(result, (self.size[0], self.size[1]))
    }

    fn mul(self: Self, i: NDArray<T, D1>) -> NDArray<T, D1> {
        let mut result = vec![];
        if cfg!(debug_assertions) {
            assert_eq!(self.size[0], i.size[0]);
        }
        for j in 0..self.size[1] {
            let mut sum: T = T::zero();
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
            let mut sum: T = T::zero();
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
    fn d1_add() {
        let vec_a = NDArray::<i32, D1>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<i32, D1>::new(vec![6, 7, 3]);
        let result = vec_a.add(vec_b);

        assert_eq!(result.data, vec![7, 9, 6]);
    }

    // #[test]
    // fn d1_m_derivatives() {
    //     let vec_a = NDArray::<f32, D1>::new(vec![1.0, 2.0]);
    //     let vec_b = NDArray::<f32, D1>::new(vec![6.0, 7.0]);
    //     let result = vec_a.jacobian(vec_b);

    //     assert_eq!(
    //         result.data,
    //         vec![1.0 / 6.0, 2.0 / 6.0, 1.0 / 7.0, 2.0 / 7.0]
    //     );
    // }
    #[test]
    fn d1_mul() {
        let vec_a = NDArray::<i32, D1>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<i32, D1>::new(vec![6, 7, 3]);
        let result = vec_a.mul(vec_b);

        assert_eq!(result.data, vec![6, 14, 9]);
    }

    #[test]
    fn d2_add() {
        let vec_a = NDArray::<i32, D2>::new(vec![1, 2, 3, 4], (2, 2));
        let vec_b = NDArray::<i32, D2>::new(vec![6, 7, 3, 8], (2, 2));
        let result = vec_a.add(vec_b);

        assert_eq!(result.data, vec![7, 9, 6, 12]);
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
