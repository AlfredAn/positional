#![recursion_limit = "1000"]

extern crate self as positional;

mod bool;
mod ops;

pub use bool::*;
pub use ops::*;

pub mod prelude {
    pub use super::*;
}

pub struct Zero;

/// Peano integer.
pub struct Successor<T>(std::marker::PhantomData<T>);

/// Integer in a positional numbering system.
pub struct Int<H, T>(std::marker::PhantomData<(H, T)>);
