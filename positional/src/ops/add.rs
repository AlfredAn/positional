use crate::prelude::*;

pub trait Add<T> {
    type Sum;
}

pub type Sum<T1, T2> = <T1 as Add<T2>>::Sum;

impl Add<PeanoZero> for PeanoZero {
    type Sum = PeanoZero;
}

impl<T> Add<PeanoSucc<T>> for PeanoZero {
    type Sum = PeanoSucc<T>;
}

impl<T> Add<PeanoZero> for PeanoSucc<T> {
    type Sum = PeanoSucc<T>;
}

impl<T1, T2> Add<PeanoSucc<T2>> for PeanoSucc<T1>
where
    T1: Add<PeanoSucc<PeanoSucc<T2>>>,
{
    type Sum = Sum<T1, PeanoSucc<PeanoSucc<T2>>>;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
    assert!(equal::<Sum<p!(0), p!(0)>, p!(0)>());
    assert!(equal::<Sum<p!(0), p!(1)>, p!(1)>());
    assert!(equal::<Sum<p!(0), p!(2)>, p!(2)>());
    assert!(equal::<Sum<p!(1), p!(0)>, p!(1)>());
    assert!(equal::<Sum<p!(1), p!(1)>, p!(2)>());
    assert!(equal::<Sum<p!(1), p!(2)>, p!(3)>());
    assert!(equal::<Sum<p!(2), p!(0)>, p!(2)>());
    assert!(equal::<Sum<p!(2), p!(1)>, p!(3)>());
    assert!(equal::<Sum<p!(2), p!(2)>, p!(4)>());
    assert!(equal::<Sum<p!(13), p!(37)>, p!(50)>());
};
