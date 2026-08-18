use crate::prelude::*;

/// Sequence terminator (represents zero)
pub struct Term<R>(std::marker::PhantomData<R>);

/// (Radix, Head, Tail)
pub struct Seq<R, H, T>(std::marker::PhantomData<(R, H, T)>);

pub trait Radix: PeanoInt + NonZero {}

impl<R> Radix for R where R: PeanoInt + NonZero {}

/// Positional integer
pub trait PosInt {
    type Radix: Radix;
}

impl<R> PosInt for Term<R>
where
    R: Radix,
{
    type Radix = R;
}

impl<R, H, T> PosInt for Seq<R, H, T>
where
    R: Radix,
    H: PeanoInt + IsLt<R>,
    T: PosInt<Radix = R>,
{
    type Radix = R;
}

/// Removes leading zeroes.
pub trait Normalize: PosInt {
    type Normalized: PosInt<Radix = Self::Radix>;
}

impl<R> Normalize for Term<R>
where
    R: Radix,
{
    type Normalized = Self;
}

pub type Normalized<T> = <T as Normalize>::Normalized;

impl<R, T> Normalize for Seq<R, PeanoZero, T>
where
    Self: PosInt,
    T: Normalize<Radix = Self::Radix>,
{
    type Normalized = Normalized<T>;
}

impl<R, H, T> Normalize for Seq<R, Successor<H>, T>
where
    R: Radix,
    Self: PosInt,
    T: PosInt<Radix = Self::Radix>,
{
    type Normalized = Self;
}
