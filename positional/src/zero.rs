use crate::prelude::*;

pub trait IsZero {}

impl IsZero for Zero {}

pub trait NonZero {}

impl<T> NonZero for Successor<T> {}

impl<R, T> IsZero for Int<R, Zero, T>
where
    Self: PosInt,
    T: IsZero,
{
}

impl<R, H, T> NonZero for Int<R, Successor<H>, T>
where
    Self: PosInt,
    T: NonZero,
{
}
