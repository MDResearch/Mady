use std::collections::binary_heap::Iter;
use std::ops::{Add, Mul, Sub};
use std::array::{IntoIter};
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
    pub fn new(len: usize, value: T) -> Tensor<T> {
        Tensor {
            data: vec![value; len],
        }
    }
}

impl<T> AsMut<Vec<T>> for Tensor<T> where T: Copy {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.data.as_mut()
    }
}

// Add
impl<K, T> Add<Tensor<K>> for Tensor<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Tensor<<T as Add<K>>::Output>;

    fn add(self, rhs: Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        }
    }
}

impl<K, T> Add<&Tensor<K>> for &Tensor<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Tensor<<T as Add<K>>::Output>;

    fn add(self, rhs: &Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        }
    }
}

// Sub
impl<K, T> Sub<Tensor<K>> for Tensor<T>
where
    T: Sub<K> + Copy,
    K: Copy,
    <T as Sub<K>>::Output: Copy,
{
    type Output = Tensor<<T as Sub<K>>::Output>;

    fn sub(self, rhs: Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
        }
    }
}

impl<K, T> Sub<&Tensor<K>> for &Tensor<T>
where
    T: Sub<K> + Copy,
    K: Copy,
    <T as Sub<K>>::Output: Copy,
{
    type Output = Tensor<<T as Sub<K>>::Output>;

    fn sub(self, rhs: &Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
        }
    }
}


// Mul
impl<K, T> Mul<Tensor<K>> for Tensor<T>
where
    T: Mul<K> + Copy,
    K: Copy,
    <T as Mul<K>>::Output: Copy,
{
    type Output = Tensor<<T as Mul<K>>::Output>;
    fn mul(self, rhs: Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a * b)
                .collect(),
        }
    }
}

impl<K, T> Mul<&Tensor<K>> for &Tensor<T>
where
    T: Mul<K> + Copy,
    K: Copy,
    <T as Mul<K>>::Output: Copy,
{
    type Output = Tensor<<T as Mul<K>>::Output>;
    fn mul(self, rhs: &Tensor<K>) -> Self::Output {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(self.data.len(),rhs.data.len())
        }

        Tensor {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a * b)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_clone() {
        assert_eq!(
            Tensor::new(5, 0),
            Tensor::new(5, 0).clone()
        );
    }

    #[test]
    fn tensor_add() {
        assert_eq!(
            Tensor::new(5, 5),
            Tensor::new(5, 1) + Tensor::new(5, 4)
        );
    }

    #[test]
    fn tensor_add_ref() {
        assert_eq!(
            Tensor::new(5, 5),
            &Tensor::new(5, 1) + &Tensor::new(5, 4)
        );
    }

    #[test]
    fn tensor_sub() {
        assert_eq!(
            Tensor::new(5, 4),
            Tensor::new(5, 5) - Tensor::new(5, 1)
        );
    }

    #[test]
    fn tensor_sub_ref() {
        assert_eq!(
            Tensor::new(5, 4),
            &Tensor::new(5, 5) - &Tensor::new(5, 1)
        );
    }
    
    #[test]
    fn tensor_mul() {
        assert_eq!(
            Tensor::new(5, 4),
            Tensor::new(5, 2) * Tensor::new(5, 2)
        );
    }
    
    #[test]
    fn tensor_mul_ref() {
        assert_eq!(
            Tensor::new(5, 4),
            &Tensor::new(5, 2) * &Tensor::new(5, 2)
        );
    }
}
