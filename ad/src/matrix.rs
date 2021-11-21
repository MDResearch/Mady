use crate::tensor::Tensor;
use std::ops::{Add, Mul, Range, Sub};

// 0  1  2
// 3  4  5
// 6  7  8
// 9 10 11
// Matrix Warpper for Tensor
#[derive(Debug, PartialEq, Clone)]
struct Matrix<T>
where
    T: Copy,
{
    shape: [usize; 2],
    data: Tensor<T>,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn new(shape: [usize; 2], value: T) -> Self {
        Matrix {
            shape,
            data: Tensor::new(shape.iter().product(), value),
        }
    }

    pub fn t(self) -> Transpose<T> {
        Transpose(self)
    }
    // faster than col iter
    pub fn into_row_iter(self) -> impl IntoIterator {
        self.data.into_iter()
    }

    pub fn into_col_iter(self) -> impl IntoIterator {
        (0..self.shape[1])
            .flat_map(move |b| (b..).step_by(self.shape[1]).take(self.shape[0]))
            .map(move |n| self.data.as_ref()[n])
    }
}

impl<T> AsMut<Vec<T>> for Matrix<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.data.as_mut()
    }
}

impl<T> AsRef<Vec<T>> for Matrix<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.data.as_ref()
    }
}

impl<K, T> Add<Matrix<K>> for Matrix<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Matrix<<T as Add<K>>::Output>;

    fn add(self, rhs: Matrix<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.shape, rhs.shape)
        }
        Matrix {
            shape: self.shape.clone(),
            data: self.data + rhs.data,
        }
    }
}

// Transpose Matrix
#[derive(Debug, PartialEq, Clone)]
struct Transpose<T>(Matrix<T>)
where
    T: Copy;

impl<T> Transpose<T>
where
    T: Copy,
{
    pub fn t(self) -> Matrix<T> {
        self.0
    }

    pub fn into_row_iter(self) -> impl IntoIterator {
        (0..self.0.shape[1])
            .flat_map(move |b| (b..).step_by(self.0.shape[1]).take(self.0.shape[0]))
            .map(move |n| self.0.data.as_ref()[n])
    }
    // faster than row iter
    pub fn into_col_iter(self) -> impl IntoIterator {
        self.0.data.into_iter()
    }
}

impl<T> AsMut<Vec<T>> for Transpose<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.0.as_mut()
    }
}

impl<T> AsRef<Vec<T>> for Transpose<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.0.as_ref()
    }
}

impl<K, T> Add<Transpose<K>> for Transpose<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Matrix<<T as Add<K>>::Output>;

    fn add(self, rhs: Transpose<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.0.shape, rhs.0.shape)
        }
        Matrix {
            shape: [self.0.shape[1], self.0.shape[0]],
            data: self.0.data + rhs.0.data,
        }
    }
}

impl<K, T> Add<&Transpose<K>> for &Transpose<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Matrix<<T as Add<K>>::Output>;

    fn add(self, rhs: &Transpose<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.0.shape, rhs.0.shape)
        }
        Matrix {
            shape: [self.0.shape[1], self.0.shape[0]],
            data: &self.0.data + &rhs.0.data,
        }
    }
}

impl<K, T> Add<Matrix<K>> for Transpose<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Matrix<<T as Add<K>>::Output>;

    fn add(self, rhs: Matrix<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!([self.0.shape[1], self.0.shape[0]], rhs.shape)
        }
        Matrix {
            shape: rhs.shape.clone(),
            data: self.0.data + rhs.data,
        }
    }
}


impl<K, T> Add<&Matrix<K>> for &Transpose<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Matrix<<T as Add<K>>::Output>;

    fn add(self, rhs: &Matrix<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!([self.0.shape[1], self.0.shape[0]], rhs.shape)
        }
        Matrix {
            shape: rhs.shape.clone(),
            data: &self.0.data + &rhs.data,
        }
    }
}
