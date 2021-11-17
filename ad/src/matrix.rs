use crate::tensor::Tensor;

#[derive(Debug, Clone)]
struct Matrix<T>
where
    T: Copy,
{
    shape: [usize; 2],
    data: Tensor<T>,
    tran: bool,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn new(shape: [usize; 2], value: T) -> Self {
        Matrix {
            shape,
            data: Tensor::new(shape.iter().product(), value),
            tran: false,
        }
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

impl<T> PartialEq for Matrix<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape && self.data == other.data
    }
}
