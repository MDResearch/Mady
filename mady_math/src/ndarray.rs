use super::grad::{GradAdd, GradMul, One, Zero};

use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NDArray<D, T> {
    phantom: PhantomData<D>,
    data: Vec<T>,
    size: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D2;

pub type Array0<T> = NDArray<D0, T>;
pub type Array1<T> = NDArray<D1, T>;
pub type Array2<T> = NDArray<D2, T>;

impl<T> Array0<T>
where
    T: Copy,
{
    pub fn new(data: T) -> Self {
        Self {
            phantom: PhantomData,
            size: vec![],
            data: vec![data],
        }
    }
}

impl<T> Array0<T>
where
    T: Zero<O0 = T> + PartialOrd + Copy,
{
    pub fn relu(self) -> Self {
        if self.data[0] < T::zero() {
            Self::new(T::zero())
        } else {
            self
        }
    }
}

impl<T> Array1<T>
where
    T: Copy,
{
    pub fn new(data: Vec<T>) -> Self {
        Self {
            phantom: PhantomData,
            size: vec![data.len()],
            data,
        }
    }
}

impl<T> Array1<T>
where
    T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero<O0 = T> + Copy,
{
    pub fn dot(self, i: Self) -> Array0<T> {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }

        NDArray::<D0, T>::new(
            self.data
                .iter()
                .zip(i.data.iter())
                .map(|(&x, &y)| x * y)
                .fold(T::zero(), |a, b| a + b),
        )
    }

    pub fn mul(self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let result = self
            .data
            .into_iter()
            .zip(i.data.into_iter())
            .map(|(a, b)| a * b)
            .collect();

        Self::new(result)
    }

    pub fn sum(self) -> Array0<T> {
        Array0::new(self.data.into_iter().fold(T::zero(), |a, x| a + x))
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

    pub fn add(self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let mut result = vec![];
        for c in 0..self.data.len() {
            result.push(self.data[c] + i.data[c]);
        }

        Self::new(result)
    }

    pub fn cross(self, i: Self) -> NDArray<T, D1> {
        // https://zh.wikipedia.org/wiki/%E7%9F%A9%E9%98%B5%E5%BE%AE%E7%A7%AF%E5%88%86
        // https://en.wikipedia.org/wiki/Matrix_calculus

        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        todo!();
    }
}

impl<T> Array2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero<O0 = T> + Copy,
{
    pub fn new(data: Vec<T>, size: (usize, usize)) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(size.0 * size.1, data.len());
        }
        Self {
            phantom: PhantomData,
            size: vec![size.0, size.1],
            data,
        }
    }

    pub fn add(self, i: Self) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(self.size, i.size);
        }
        let mut result = vec![];
        for c in 0..self.data.len() {
            result.push(self.data[c] + i.data[c]);
        }

        Self::new(result, (self.size[0], self.size[1]))
    }

    pub fn mul(self, i: Array1<T>) -> Array1<T> {
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

        NDArray::<D1, T>::new(result)
    }

    // fn grad_mul(self, i: NDArray<T, D1>) -> (NDArray<T, D1>, (NDArray<T, D2>, NDArray<T, D1>)) {
    //     let mut result = vec![];
    //     if cfg!(debug_assertions) {
    //         assert_eq!(self.size[0], i.size[0]);
    //     }
    //     for j in 0..self.size[1] {
    //         let mut sum: T = T::zero();
    //         for c in 0..self.size[0] {
    //             sum = sum + self.data[j * self.size[0] + c] * i.data[c];
    //         }
    //         result.push(sum);
    //     }

    //     NDArray::<T, D1>::new(result);
    //     todo!()
    // }
}

pub trait GradDot<Rhs = Self> {
    type O0;
    type G0;
    type G1;
    fn grad_dot(self, rhs: Rhs) -> (Self::O0, (Self::G0, Self::G1));
}

pub trait GradRelu {
    type O0;
    type G0;
    fn grad_relu(self) -> (Self::O0, (Self::G0,));
}

impl<T> Zero for Array0<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Array0<T>;

    fn zero() -> Self::O0 {
        Self {
            phantom: PhantomData,
            data: vec![],
            size: vec![],
        }
    }
}

impl<T> One for Array0<T>
where
    T: Zero<O0 = T> + One<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Array0<T>;

    fn one() -> Self::O0 {
        Self::O0::new(T::one())
    }
}

impl<T> Zero for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Array1<T>;

    fn zero() -> Self::O0 {
        Self {
            phantom: PhantomData,
            data: vec![],
            size: vec![],
        }
    }
}

impl<T> One for Array1<T>
where
    T: Zero<O0 = T> + One<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Array1<T>;

    fn one() -> Self::O0 {
        Self {
            phantom: PhantomData,
            data: vec![],
            size: vec![],
        }
    }
}

impl<T> GradMul for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Self;
    type G0 = Self;
    type G1 = Self;

    fn grad_mul(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
        (self.clone().mul(rhs.clone()), (rhs, self))
    }
}

impl<T> GradAdd for Array0<T>
where
    T: Zero<O0 = T> + One<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Self;
    type G0 = Self;
    type G1 = Self;

    fn grad_add(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
        (
            Self::new(self.data[0] + rhs.data[0]),
            (Self::new(T::one()), Self::new(T::one())),
        )
    }
}

impl<T> GradDot for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type O0 = Array0<T>;
    type G0 = Self;
    type G1 = Self;

    fn grad_dot(self, rhs: Self) -> (Self::O0, (Self::G0, Self::G1)) {
        (self.clone().dot(rhs.clone()), (rhs, self))
    }
}

impl<T> GradRelu for Array0<T>
where
    T: One<O0 = T> + Zero<O0 = T> + PartialOrd + Copy,
{
    type O0 = Self;
    type G0 = Self;

    fn grad_relu(self) -> (Self::O0, (Self::G0,)) {
        if self.data[0] < T::zero() {
            (Self::new(T::zero()), (Self::new(T::zero()),))
        } else {
            (self, (Self::new(T::one()),))
        }
    }
}

impl<T> Mul for Array0<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.data[0] * rhs.data[0])
    }
}

impl<T> Mul<Array0<T>> for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Array0<T>) -> Self::Output {
        Self::new(self.clone().data.into_iter().map(|x| x * *rhs).collect())
    }
}

impl<T> Mul<Array1<T>> for Array0<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type Output = Array1<T>;

    fn mul(self, rhs: Array1<T>) -> Self::Output {
        if self.data.is_empty(){
            Array1::<T>::zero()
        }else {
            Self::Output::new(rhs.data.into_iter().map(|x| x * *self).collect())
        }
    }
}

impl<T> Add<Array0<T>> for Array0<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type Output = Array0<T>;

    fn add(self, rhs: Array0<T>) -> Self::Output {
        Self::Output::new(rhs.data[0] + self.data[0])
    }
}

impl<T> Add for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    type Output = Array1<T>;

    fn add(self, rhs: Array1<T>) -> Self::Output {
        Self::Output::new(
            rhs.data
                .into_iter()
                .zip(self.data.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<D, T> AddAssign for NDArray<D, T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T> + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        if self.data.is_empty() {
            *self = rhs;
        } else {
            if cfg!(debug_assertions) {
                assert_eq!(self.size, rhs.size);
            }
            self.data
                .iter_mut()
                .zip(rhs.data.into_iter())
                .for_each(|(a, b)| *a += b)
        }
    }
}

impl<T> Sub for Array1<T>
where
    T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.data
                .into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl<T> Index<usize> for Array1<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Array1<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Deref for Array0<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data[0]
    }
}

impl<T> DerefMut for Array0<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[0]
    }
}

impl<T> From<Vec<T>> for Array1<T>
where
    T: Copy,
{
    fn from(d: Vec<T>) -> Self {
        Self::new(d)
    }
}

impl<D, T> IntoIterator for NDArray<D, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// impl<T> Mul<Array0<T>> for Array1<T>
// where
//     T: Zero<O0 = T> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
// {
//     type Output = Array1<T>;

//     fn mul(self, rhs: Array0<T>) -> Self::Output {
//         NDArray::<D1, T>::new(
//             self.clone()
//                 .data
//                 .into_iter()
//                 .map(|x| x * rhs.data[0])
//                 .collect(),
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_dot() {
        let vec_a = NDArray::<D1, i32>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<D1, i32>::new(vec![6, 7, 3]);
        let result = vec_a.dot(vec_b);

        assert_eq!(result, NDArray::<D0, _>::new(29_i32));
    }

    #[test]
    fn d1_add() {
        let vec_a = NDArray::<D1, i32>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<D1, i32>::new(vec![6, 7, 3]);
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
        let vec_a = NDArray::<D1, i32>::new(vec![1, 2, 3]);
        let vec_b = NDArray::<D1, i32>::new(vec![6, 7, 3]);
        let result = vec_a.mul(vec_b);

        assert_eq!(result.data, vec![6, 14, 9]);
    }

    #[test]
    fn d2_add() {
        let vec_a = NDArray::<D2, i32>::new(vec![1, 2, 3, 4], (2, 2));
        let vec_b = NDArray::<D2, i32>::new(vec![6, 7, 3, 8], (2, 2));
        let result = vec_a.add(vec_b);

        assert_eq!(result.data, vec![7, 9, 6, 12]);
    }

    #[test]
    fn d2_mul_d1() {
        let vec_a = NDArray::<D2, i32>::new(vec![1, 2, 3, 4], (2, 2));
        let vec_b = NDArray::<D1, i32>::new(vec![7, 1]);
        let result = vec_a.mul(vec_b);

        assert_eq!(result.data[0], 9);
        assert_eq!(result.data[1], 25);
    }
}
