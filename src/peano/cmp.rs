use crate::{
    bool::{Bool, False, True},
    peano::{One, Successor, Two, Zero},
};

pub trait Lt<T> {
    type Lt: Bool;
}

impl Lt<Zero> for Zero {
    type Lt = False;
}

impl<T> Lt<Successor<T>> for Zero {
    type Lt = True;
}

impl<T> Lt<Zero> for Successor<T> {
    type Lt = False;
}

impl<T1, T2> Lt<Successor<T2>> for Successor<T1>
where
    T1: Lt<T2>,
{
    type Lt = <T1 as Lt<T2>>::Lt;
}

pub const fn lt<T1, T2>() -> bool
where
    T1: Lt<T2>,
{
    <T1 as Lt<T2>>::Lt::VALUE
}

pub(crate) trait NotLt<T>: Lt<T, Lt = False> {}
pub(crate) trait IsLt<T>: Lt<T, Lt = True> {}

impl<T1, T2> NotLt<T2> for T1 where T1: Lt<T2, Lt = False> {}
impl<T1, T2> IsLt<T2> for T1 where T1: Lt<T2, Lt = True> {}

const _: () = const {
    assert!(!lt::<Zero, Zero>());
    assert!(lt::<Zero, One>());
    assert!(lt::<Zero, Two>());
    assert!(!lt::<One, Zero>());
    assert!(!lt::<One, One>());
    assert!(lt::<One, Two>());
    assert!(!lt::<Two, Zero>());
    assert!(!lt::<Two, One>());
    assert!(!lt::<Two, Two>());
};
