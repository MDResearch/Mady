pub mod grad;
pub mod matrix;
pub mod tensor;
#[macro_use]
mod macros;

pub mod prelude {
    pub use super::grad::*;
    pub use super::matrix::{mat, Matrix};
    pub use super::tensor::{ten, Tensor};
}