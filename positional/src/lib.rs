#![recursion_limit = "1000"]

extern crate self as positional;

mod bool;
mod ops;
mod peano;
mod pos;
mod zero;

pub use bool::*;
pub use ops::*;
pub use peano::*;
pub use pos::*;
pub use zero::*;

pub mod prelude {
    pub use super::*;
}
