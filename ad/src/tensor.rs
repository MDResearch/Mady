use std::{
    ops::{Add, Mul, Sub},
    vec::IntoIter,
};

use crate::{impl_ops, impl_ops_all, impl_trait, test_ops, test_ops_all};
#[derive(Debug, PartialEq, Clone)]
pub struct Tensor<T>
where
    T: Copy,
{
    data: Vec<T>,
}

impl<T> Tensor<T>
where
    T: Copy,
{
    pub fn new(len: usize, value: T) -> Self {
        Tensor {
            data: vec![value; len],
        }
    }
}

impl<T> IntoIterator for Tensor<T>
where
    T: Copy,
{
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> AsMut<Vec<T>> for Tensor<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.data.as_mut()
    }
}

impl<T> AsRef<Vec<T>> for Tensor<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.data.as_ref()
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
//             assert_eq!(self.data.len(), rhs.data.len())
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
impl_ops_all!(+[<K,T> where T: Copy,T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Add<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.data.len(), right.data.len())
        }
        Tensor {
            data: left
                .data
                .iter()
                .zip(right.data.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        }
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
//             assert_eq!(self.data.len(), rhs.data.len())
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
impl_ops_all!(-[<K,T> where T: Copy,T: Sub<K> + Copy,K: Copy,<T as Sub<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Sub<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.data.len(), right.data.len())
        }
        Tensor {
            data:left
                .data
                .iter()
                .zip(right.data.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
        }
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
//             assert_eq!(self.data.len(), rhs.data.len())
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
impl_ops_all!(*[<K,T> where T: Copy,T: Mul<K> + Copy,K: Copy,<T as Mul<K>>::Output: Copy,]
    (left:Tensor<T>,right:Tensor<K>)->Tensor<<T as Mul<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.data.len(), right.data.len())
        }
        Tensor {
            data:left
                .data
                .iter()
                .zip(right.data.iter())
                .map(|(&a, &b)| a * b)
                .collect(),
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_clone() {
        assert_eq!(Tensor::new(5, 0), Tensor::new(5, 0).clone());
    }

    #[test]
    fn tensor_add() {
        test_ops_all!(=,+,Tensor::new(5, 1),Tensor::new(5, 4),Tensor::new(5, 5));
    }

    #[test]
    fn tensor_sub() {
        test_ops_all!(=,-,Tensor::new(5, 5),Tensor::new(5, 1),Tensor::new(5, 4));
    }

    #[test]
    fn tensor_mul() {
        test_ops_all!(=,*,Tensor::new(5, 2),Tensor::new(5, 3),Tensor::new(5, 6));
    }
}
