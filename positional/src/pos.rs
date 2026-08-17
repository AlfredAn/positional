use crate::prelude::*;

/// (Radix, Head, Tail)
pub struct Int<R, H, T>(std::marker::PhantomData<(R, H, T)>);

/// Positional integer
pub trait PosInt {}

impl PosInt for Zero {}

impl<R, H, T> PosInt for Int<R, H, T>
where
    R: PeanoInt + NonZero,
    H: PeanoInt + IsLt<R>,
    T: PosInt,
{
}

/// Removes leading zeroes.
pub trait Normalize: PosInt {
    type Normalized: PosInt;
}

impl Normalize for Zero {
    type Normalized = Zero;
}

pub type Normalized<T> = <T as Normalize>::Normalized;

impl<R, T> Normalize for Int<R, Zero, T>
where
    Self: PosInt,
    T: Normalize,
{
    type Normalized = Normalized<T>;
}

impl<R, H, T> Normalize for Int<R, Successor<H>, T>
where
    Self: PosInt,
    T: PosInt,
{
    type Normalized = Self;
}
