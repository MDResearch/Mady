pub mod tensor;

#[cfg(test)]
mod tests {
    use crate::tensor::Tensor;
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
