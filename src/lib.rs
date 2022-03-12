pub mod prelude {
    #[cfg(feature = "macro")]
    pub use mady_macro::*;
    #[cfg(feature = "math")]
    pub use mady_math::prelude::*;
    #[cfg(feature = "macro")]
    pub use std::ops::Mul;
}

pub use mady_macro as _;
pub use mady_math as math;
