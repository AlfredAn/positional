use crate::{
    bool::{Bool, False, True},
    peano::{One, Successor, Two, Zero},
};

pub trait Equal<T> {
    type Equal: Bool;
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

pub const fn equal<T1, T2>() -> bool
where
    T1: Equal<T2>,
{
    <T1 as Equal<T2>>::Equal::VALUE
}

const _: () = const {
    assert!(equal::<Zero, Zero>());
    assert!(!equal::<Zero, One>());
    assert!(!equal::<Zero, Two>());
    assert!(!equal::<One, Zero>());
    assert!(equal::<One, One>());
    assert!(!equal::<One, Two>());
    assert!(!equal::<Two, Zero>());
    assert!(!equal::<Two, One>());
    assert!(equal::<Two, Two>());
};
