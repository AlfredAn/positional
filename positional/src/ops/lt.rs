use crate::prelude::*;

pub trait Lt<T> {
    type Lt: Bool;
}

pub trait NotLt<T>: Lt<T, Lt = False> {}
pub trait IsLt<T>: Lt<T, Lt = True> {}

impl<T1, T2> NotLt<T2> for T1 where T1: Lt<T2, Lt = False> {}
impl<T1, T2> IsLt<T2> for T1 where T1: Lt<T2, Lt = True> {}

pub const fn lt<T1, T2>() -> bool
where
    T1: Lt<T2>,
{
    <T1 as Lt<T2>>::Lt::VALUE
}

impl Lt<PeanoZero> for PeanoZero {
    type Lt = False;
}

impl<T> Lt<PeanoSucc<T>> for PeanoZero {
    type Lt = True;
}

impl<T> Lt<PeanoZero> for PeanoSucc<T> {
    type Lt = False;
}

impl<T1, T2> Lt<PeanoSucc<T2>> for PeanoSucc<T1>
where
    T1: Lt<T2>,
{
    type Lt = <T1 as Lt<T2>>::Lt;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
    assert!(!lt::<p!(0), p!(0)>());
    assert!(lt::<p!(0), p!(1)>());
    assert!(lt::<p!(0), p!(2)>());
    assert!(!lt::<p!(1), p!(0)>());
    assert!(!lt::<p!(1), p!(1)>());
    assert!(lt::<p!(1), p!(2)>());
    assert!(!lt::<p!(2), p!(0)>());
    assert!(!lt::<p!(2), p!(1)>());
    assert!(!lt::<p!(2), p!(2)>());
    assert!(lt::<p!(30), p!(31)>());
    assert!(!lt::<p!(30), p!(30)>());
};
