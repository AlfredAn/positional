use crate::prelude::*;

/// Sequence terminator (represents zero)
pub struct Term<R>(std::marker::PhantomData<R>);

/// (Radix, Head, Tail)
///
/// Head is the least significant digit.
///
/// Tail is the rest of the digits.
pub struct Seq<R, H, T>(std::marker::PhantomData<(R, H, T)>);

impl<R> Default for Term<R> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<R, H, T> Default for Seq<R, H, T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

/// This type is valid to use as a radix.
pub trait Radix: PeanoInt {}

impl<R> Radix for PeanoSucc<PeanoSucc<R>> where R: PeanoInt {}

/// This trait can be used to assert that a type is a valid positional integer.
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
