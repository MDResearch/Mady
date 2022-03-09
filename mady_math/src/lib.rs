pub mod grad;
pub mod matrix;
pub mod tensor;
#[macro_use]
mod macros;

pub mod prelude {
    pub use super::grad::*;
    pub use super::grad::impl_one as _;
    pub use super::grad::impl_zero as _;
    pub use super::matrix::{mat, Matrix};
    pub use super::tensor::{ten, Tensor};
}
