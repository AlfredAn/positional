#![recursion_limit = "1000"]

extern crate self as positional;

mod bool;
mod zero;
mod ops;
mod peano;
mod pos;

pub use bool::*;
pub use zero::*;
pub use ops::*;
pub use peano::*;
pub use pos::*;

pub mod prelude {
    pub use super::*;
}

pub struct Zero;
