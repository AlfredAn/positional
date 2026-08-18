use crate::prelude::*;

pub trait IsZero {}

pub trait NonZero {}

impl IsZero for PeanoZero {}

impl<T> NonZero for PeanoSucc<T> {}

impl<R, T> IsZero for Seq<R, PeanoZero, T>
where
    Self: PosInt,
    T: PosInt<Radix = R> + IsZero,
{
}

impl<R, H, T> NonZero for Seq<R, PeanoSucc<H>, T>
where
    Self: PosInt,
    T: PosInt<Radix = R>,
{
}
