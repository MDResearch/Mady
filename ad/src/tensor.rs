use std::ops::{Add,Sub};

#[derive(Debug,PartialEq)]
pub struct Tensor<T> where T:Clone{
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T> Tensor<T>  where T:Clone{
    pub fn new(shape: Vec<usize>,value:T) -> Tensor<T> {
        Tensor {
            data: vec![value;shape.iter().product()],
            shape: shape,
        }
    }
}

impl<K,T> Add<Tensor<K>> for Tensor<T> where T:Add<K>+Copy,K:Copy,<T as Add<K>>::Output:Copy{
    type Output=Tensor<<T as Add<K>>::Output>;

    fn add(self, rhs: Tensor<K>) -> Self::Output {
        Tensor{
            data: self.data.iter().zip(rhs.data.iter()).map(|(&a,&b)|a+b).collect(),
            shape: self.shape.clone(),
        }
    }
}

impl<K,T> Add<&Tensor<K>> for &Tensor<T> where T:Add<K>+Copy,K:Copy,<T as Add<K>>::Output:Copy{
    type Output=Tensor<<T as Add<K>>::Output>;

    fn add(self, rhs: &Tensor<K>) -> Self::Output {
        Tensor{
            data: self.data.iter().zip(rhs.data.iter()).map(|(&a,&b)|a+b).collect(),
            shape: self.shape.clone(),
        }
    }
}


impl<K,T> Sub<Tensor<K>> for Tensor<T> where T:Sub<K>+Copy,K:Copy,<T as Sub<K>>::Output:Copy{
    type Output=Tensor<<T as Sub<K>>::Output>;

    fn sub(self, rhs: Tensor<K>) -> Self::Output {
        Tensor{
            data: self.data.iter().zip(rhs.data.iter()).map(|(&a,&b)|a-b).collect(),
            shape: self.shape.clone(),
        }
    }
}

impl<K,T> Sub<&Tensor<K>> for &Tensor<T> where T:Sub<K>+Copy,K:Copy,<T as Sub<K>>::Output:Copy{
    type Output=Tensor<<T as Sub<K>>::Output>;

    fn sub(self, rhs: &Tensor<K>) -> Self::Output {
        Tensor{
            data: self.data.iter().zip(rhs.data.iter()).map(|(&a,&b)|a-b).collect(),
            shape: self.shape.clone(),
        }
    }
}
