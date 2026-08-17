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

impl<T> Equal<Successor<T>> for Zero {
    type Equal = False;
}

impl<T> Equal<Zero> for Successor<T> {
    type Equal = False;
}

impl<T1, T2> Equal<Successor<T2>> for Successor<T1>
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

impl NormEqual<Zero> for Zero {
    type Equal = True;
}

impl<R, H, T> NormEqual<Int<R, H, T>> for Zero
where
    Int<R, H, T>: PosInt,
{
    type Equal = False;
}

impl<R, H, T> NormEqual<Zero> for Int<R, H, T>
where
    Self: PosInt,
{
    type Equal = False;
}

impl<R, H1, H2, T1, T2> NormEqual<Int<R, H2, T2>> for Int<R, H1, T1>
where
    Self: PosInt,
    Int<R, H2, T2>: PosInt,
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
    use positional_macro::peano as p;
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
