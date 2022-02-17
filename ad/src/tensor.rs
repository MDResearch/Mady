//! define the datastructure,tensor.
//! describe all the implement of tensor.

use std::ops::{Add, Mul, Sub};

use crate::{impl_ops, impl_ops_all, impl_ops_trait};

pub use crate::ten;

#[derive(Debug, PartialEq, Clone)]
pub struct Tensor<T>(Vec<T>)
where
    T: Copy;

impl<T> Tensor<T>
where
    T: Copy,
{
    pub fn new(value: T, len: usize) -> Self {
        Tensor(vec![value; len])
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut T> + 'a {
        self.0.iter_mut()
    }
}

impl<T> From<Vec<T>> for Tensor<T>
where
    T: Copy,
{
    fn from(data: Vec<T>) -> Self {
        Tensor(data)
    }
}

impl<T> AsMut<Vec<T>> for Tensor<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.0.as_mut()
    }
}

impl<T> AsRef<Vec<T>> for Tensor<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.0.as_ref()
    }
}

// Add
// impl<K, T> Add<Tensor<K>> for Tensor<T>
// where
//     T: Add<K> + Copy,
//     K: Copy,
//     <T as Add<K>>::Output: Copy,
// {
//     type Output = Tensor<<T as Add<K>>::Output>;

//     fn add(self, rhs: Tensor<K>) -> Self::Output {
//         if cfg!(debug_assertions) {
//             assert_eq!(self.0.len(), rhs.data.len())
//         }

//         Tensor {
//             data: self
//                 .data
//                 .iter()
//                 .zip(rhs.data.iter())
//                 .map(|(&a, &b)| a + b)
//                 .collect(),
//         }
//     }
// }
impl_ops_all!(+[<K, T> where T: Copy,T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Add<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.len(), right.0.len())
        }
        Tensor(
            left
                .0
                .iter()
                .zip(right.0.iter())
                .map(|(&a, &b)| a + b)
                .collect()
        )
    }
);

// Sub
// impl<K, T> Sub<Tensor<K>> for Tensor<T>
// where
//     T: Sub<K> + Copy,
//     K: Copy,
//     <T as Sub<K>>::Output: Copy,
// {
//     type Output = Tensor<<T as Sub<K>>::Output>;

//     fn sub(self, rhs: Tensor<K>) -> Self::Output {
//         if cfg!(debug_assertions) {
//             assert_eq!(self.0.len(), rhs.data.len())
//         }

//         Tensor {
//             data: self
//                 .data
//                 .iter()
//                 .zip(rhs.data.iter())
//                 .map(|(&a, &b)| a - b)
//                 .collect(),
//         }
//     }
// }
impl_ops_all!(-[<K, T> where T: Copy,T: Sub<K> + Copy,K: Copy,<T as Sub<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Sub<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.len(), right.0.len())
        }
        Tensor(
            left
                .0
                .iter()
                .zip(right.0.iter())
                .map(|(&a, &b)| a - b)
                .collect()
        )
    }
);

// Mul
// impl<K, T> Mul<Tensor<K>> for Tensor<T>
// where
//     T: Mul<K> + Copy,
//     K: Copy,
//     <T as Mul<K>>::Output: Copy,
// {
//     type Output = Tensor<<T as Mul<K>>::Output>;
//     fn mul(self, rhs: Tensor<K>) -> Self::Output {
//         if cfg!(debug_assertions) {
//             assert_eq!(self.0.len(), rhs.data.len())
//         }

//         Tensor {
//             data: self
//                 .data
//                 .iter()
//                 .zip(rhs.data.iter())
//                 .map(|(&a, &b)| a * b)
//                 .collect(),
//         }
//     }
// }
impl_ops_all!(*[<K, T> where T: Copy,T: Mul<K> + Copy,K: Copy,<T as Mul<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Mul<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.len(), right.0.len())
        }
        Tensor (
            left
                .0
                .iter()
                .zip(right.0.iter())
                .map(|(&a, &b)| a * b)
                .collect(),

        )
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ten, test_ops, test_ops_all};

    #[test]
    fn clone() {
        assert_eq!(Tensor::new(0, 5), Tensor::new(0, 5).clone());
    }

    #[test]
    fn add() {
        test_ops_all!(=,+,Tensor::new(1, 5),Tensor::new(4, 5),Tensor::new(5, 5));
    }

    #[test]
    fn sub() {
        test_ops_all!(=,-,Tensor::new(5, 5),Tensor::new(1, 5),Tensor::new(4, 5));
    }

    #[test]
    fn mul() {
        test_ops_all!(=,*,Tensor::new(2, 5),Tensor::new(3, 5),Tensor::new(6, 5));
    }

    #[test]
    fn macro_with_size() {
        assert_eq!(ten![1;10], Tensor::new(1, 10))
    }

    #[test]
    fn macro_with_vec() {
        assert_eq!(ten![1, 2, 3, 4, 5, 6], Tensor::from(vec![1, 2, 3, 4, 5, 6]))
    }
}
