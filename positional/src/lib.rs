#![recursion_limit = "1000"]

extern crate self as positional;

pub mod peano;
pub mod pos;

mod bool;
mod traits;

pub use bool::{False, True};
pub use traits::*;

pub struct Zero;
