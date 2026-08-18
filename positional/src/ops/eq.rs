use crate::prelude::*;

pub trait Equal<T> {
    type Equal: Bool;
}

pub trait NotEq<T>: Equal<T, Equal = False> {}
pub trait IsEq<T>: Equal<T, Equal = True> {}

impl<T1, T2> NotEq<T2> for T1 where T1: Equal<T2, Equal = False> {}
impl<T1, T2> IsEq<T2> for T1 where T1: Equal<T2, Equal = True> {}

pub type Eq<T1, T2> = <T1 as Equal<T2>>::Equal;

pub const fn equal<T1, T2>() -> bool
where
    T1: Equal<T2>,
{
    <Eq<T1, T2>>::VALUE
}

impl Equal<PeanoZero> for PeanoZero {
    type Equal = True;
}

impl<T> Equal<PeanoSucc<T>> for PeanoZero {
    type Equal = False;
}

impl<T> Equal<PeanoZero> for PeanoSucc<T> {
    type Equal = False;
}

impl<T1, T2> Equal<PeanoSucc<T2>> for PeanoSucc<T1>
where
    T1: Equal<T2>,
{
    type Equal = Eq<T1, T2>;
}

mod hidden {
    use crate::Bool;

    /// Only works for normalized positional integers.
    pub trait NormEqual<T> {
        type Equal: Bool;
    }
}

use hidden::NormEqual;

type NormEq<T1, T2> = <T1 as NormEqual<T2>>::Equal;

impl<R> NormEqual<Term<R>> for Term<R>
where
    R: Radix,
{
    type Equal = True;
}

impl<R, H, T> NormEqual<Seq<R, H, T>> for Term<R>
where
    Seq<R, H, T>: PosInt,
{
    type Equal = False;
}

impl<R, H, T> NormEqual<Term<R>> for Seq<R, H, T>
where
    Self: PosInt,
{
    type Equal = False;
}

impl<R, H1, H2, T1, T2> NormEqual<Seq<R, H2, T2>> for Seq<R, H1, T1>
where
    Self: PosInt,
    Seq<R, H2, T2>: PosInt,
    H1: Equal<H2>,
    T1: NormEqual<T2>,
    Eq<H1, H2>: And<NormEq<T1, T2>>,
{
    type Equal = Both<Eq<H1, H2>, NormEq<T1, T2>>;
}

impl<T1, T2> Equal<T2> for T1
where
    T1: Normalize,
    T2: Normalize,
    T1::Normalized: NormEqual<T2::Normalized>,
{
    type Equal = NormEq<Normalized<T1>, Normalized<T2>>;
}

const _: () = const {
    use peano as p;
    assert!(equal::<p!(0), p!(0)>());
    assert!(!equal::<p!(0), p!(1)>());
    assert!(!equal::<p!(0), p!(2)>());
    assert!(!equal::<p!(1), p!(0)>());
    assert!(equal::<p!(1), p!(1)>());
    assert!(!equal::<p!(1), p!(2)>());
    assert!(!equal::<p!(2), p!(0)>());
    assert!(!equal::<p!(2), p!(1)>());
    assert!(equal::<p!(2), p!(2)>());
};
