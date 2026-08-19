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

impl<R> Equal<Term<R>> for Term<R> {
    type Equal = True;
}

impl<R, T> Equal<Seq<R, PeanoZero, T>> for Term<R>
where
    Term<R>: Equal<T>,
{
    type Equal = Eq<Term<R>, T>;
}

impl<R, T> Equal<Term<R>> for Seq<R, PeanoZero, T>
where
    T: Equal<Term<R>>,
{
    type Equal = Eq<T, Term<R>>;
}

impl<R, H, T> Equal<Seq<R, PeanoSucc<H>, T>> for Term<R> {
    type Equal = False;
}

impl<R, H, T> Equal<Term<R>> for Seq<R, PeanoSucc<H>, T> {
    type Equal = False;
}

impl<R, H1, H2, T1, T2> Equal<Seq<R, H2, T2>> for Seq<R, H1, T1>
where
    H1: Equal<H2>,
    T1: Equal<T2>,
    Eq<H1, H2>: And<Eq<T1, T2>>,
{
    type Equal = Both<Eq<H1, H2>, Eq<T1, T2>>;
}

#[cfg(test)]
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

    define_encoding!(bin, "01");
    assert!(equal::<number!(bin, "0"), number!(bin, "0")>());
    assert!(equal::<number!(bin, ""), number!(bin, "")>());
    assert!(equal::<number!(bin, ""), number!(bin, "0")>());
    assert!(equal::<number!(bin, "0"), number!(bin, "000")>());
    assert!(!equal::<number!(bin, "0"), number!(bin, "1")>());
    assert!(equal::<number!(bin, "1"), number!(bin, "01")>());
    assert!(equal::<number!(bin, "10101001"), number!(bin, "10101001")>());
    assert!(!equal::<number!(bin, "10101001"), number!(bin, "10101000")>());
    assert!(equal::<number!(bin, "010101001"), number!(bin, "0010101001")>());
};
