use crate::prelude::*;

/// Sequence terminator (represents zero)
pub struct Term<R>(std::marker::PhantomData<R>)
where
    R: Radix;

/// (Radix, Head, Tail)
///
/// Head is the least significant digit.
///
/// Tail is the rest of the digits.
pub struct Seq<R, H, T>(std::marker::PhantomData<(R, H, T)>)
where
    Self: PosInt;

impl<R> Default for Term<R>
where
    R: Radix,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<R, H, T> Default for Seq<R, H, T>
where
    Self: PosInt,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

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

impl<R, H, T> Normalize for Seq<R, PeanoSucc<H>, T>
where
    R: Radix,
    Self: PosInt,
    T: PosInt<Radix = Self::Radix>,
{
    type Normalized = Self;
}

// /// Zeroes all digits of a number.
// trait MakeZeroed: PosInt {
//     type Zeroed: PosInt<Radix = Self::Radix>;
// }

// type Zeroed<T> = <T as MakeZeroed>::Zeroed;

// impl<R> MakeZeroed for Term<R>
// where
//     R: Radix,
// {
//     type Zeroed = Term<R>;
// }

// impl<R, H, T> MakeZeroed for Seq<R, H, T>
// where
//     R: Radix,
//     Self: PosInt<Radix = R>,
//     T: PosInt<Radix = R> + MakeZeroed,
//     PeanoZero: IsLt<R>,
// {
//     type Zeroed = Seq<R, PeanoZero, Zeroed<T>>;
// }

// /// Pads the shorter of two numbers with zeroes to make them the same length.
// pub trait SameLength<T>: PosInt
// where
//     T: PosInt<Radix = Self::Radix>,
// {
//     type This: PosInt<Radix = Self::Radix>;
//     type Other: PosInt<Radix = Self::Radix>;
// }

// impl<R> SameLength<Term<R>> for Term<R>
// where
//     R: Radix,
// {
//     type This = Term<R>;
//     type Other = Term<R>;
// }

// impl<R, H, T> SameLength<Seq<R, H, T>> for Term<R>
// where
//     R: Radix,
//     Seq<R, H, T>: PosInt<Radix = R>,
// {
//     type This = Zeroed<Seq<R, H, T>>;
//     type Other = Seq<R, H, T>;
// }
// wrap one step at a time maybe?
