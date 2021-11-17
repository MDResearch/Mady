use std::ops::{Add, Mul, Sub};

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
    pub fn new(shape: Vec<usize>, value: T) -> Tensor<T> {
        Tensor {
            data: vec![value; shape.iter().product()],
        }
    }
}

impl<K, T> Add<Tensor<K>> for Tensor<T>
where
    T: Add<K> + Copy,
    K: Copy,
    <T as Add<K>>::Output: Copy,
{
    type Output = Tensor<<T as Add<K>>::Output>;

    fn add(self, rhs: Tensor<K>) -> Self::Output {
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

impl<K, T> Sub<Tensor<K>> for Tensor<T>
where
    T: Sub<K> + Copy,
    K: Copy,
    <T as Sub<K>>::Output: Copy,
{
    type Output = Tensor<<T as Sub<K>>::Output>;

    fn sub(self, rhs: Tensor<K>) -> Self::Output {
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

impl<K, T> Mul<Tensor<K>> for Tensor<T>
where
    T: Mul<K> + Copy,
    K: Copy,
    <T as Mul<K>>::Output: Copy,
{
    type Output = Tensor<<T as Mul<K>>::Output>;
    fn mul(self, rhs: Tensor<K>) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_clone() {
        assert_eq!(
            Tensor::new(vec!(2, 3), 0),
            Tensor::new(vec!(2, 3), 0).clone()
        );
    }

    #[test]
    fn tensor_add() {
        assert_eq!(
            Tensor::new(vec!(2, 3), 5),
            Tensor::new(vec!(2, 3), 1) + Tensor::new(vec!(2, 3), 4)
        );
    }

    #[test]
    fn tensor_add_ref() {
        assert_eq!(
            Tensor::new(vec!(2, 3), 5),
            &Tensor::new(vec!(2, 3), 1) + &Tensor::new(vec!(2, 3), 4)
        );
    }

    #[test]
    fn tensor_sub() {
        assert_eq!(
            Tensor::new(vec!(2, 3), 4),
            Tensor::new(vec!(2, 3), 5) - Tensor::new(vec!(2, 3), 1)
        );
    }

    #[test]
    fn tensor_sub_ref() {
        assert_eq!(
            Tensor::new(vec!(2, 3), 4),
            &Tensor::new(vec!(2, 3), 5) - &Tensor::new(vec!(2, 3), 1)
        );
    }
}
