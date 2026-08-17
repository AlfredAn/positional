use crate::{
    bool::{Bool, False, True},
    peano::{Successor, Zero},
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
    use positional_macro::peano as p;
    assert!(!lt::<p!(0), p!(0)>());
    assert!(lt::<p!(0), p!(1)>());
    assert!(lt::<p!(0), p!(2)>());
    assert!(!lt::<p!(1), p!(0)>());
    assert!(!lt::<p!(1), p!(1)>());
    assert!(lt::<p!(1), p!(2)>());
    assert!(!lt::<p!(2), p!(0)>());
    assert!(!lt::<p!(2), p!(1)>());
    assert!(!lt::<p!(2), p!(2)>());
};
