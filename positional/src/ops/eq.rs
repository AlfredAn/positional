use crate::prelude::*;

pub trait Equal<T> {
    type Equal: Bool;
}

pub const fn equal<T1, T2>() -> bool
where
    T1: Equal<T2>,
{
    <T1 as Equal<T2>>::Equal::VALUE
}

impl Equal<Zero> for Zero {
    type Equal = True;
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
    type Equal = <T1 as Equal<T2>>::Equal;
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
