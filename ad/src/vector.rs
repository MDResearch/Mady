use crate::tensor::{Tensor};

struct Vector<T> where T:Copy{
    data:Tensor<T>
}