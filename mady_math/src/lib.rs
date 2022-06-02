#[cfg(feature = "nalgebra")]
pub mod nalgebras;
pub mod stds;
#[macro_use]
mod macros;

pub mod prelude {
    pub use super::stds::*;

    #[cfg(feature = "nalgebra")]
    pub use super::nalgebras::*;
}
