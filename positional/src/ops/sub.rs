use crate::prelude::*;

pub trait Subtract<T> {
    type Difference;
}

pub type Difference<T1, T2> = <T1 as Subtract<T2>>::Difference;

impl Subtract<PeanoZero> for PeanoZero {
    type Difference = PeanoZero;
}

impl<T> Subtract<PeanoZero> for PeanoSucc<T> {
    type Difference = PeanoSucc<T>;
}

impl<T1, T2> Subtract<PeanoSucc<T2>> for PeanoSucc<T1>
where
    T1: Subtract<T2>,
{
    type Difference = Difference<T1, T2>;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
    assert!(equal::<Difference<p!(0), p!(0)>, p!(0)>());
    assert!(equal::<Difference<p!(1), p!(0)>, p!(1)>());
    assert!(equal::<Difference<p!(1), p!(1)>, p!(0)>());
    assert!(equal::<Difference<p!(2), p!(0)>, p!(2)>());
    assert!(equal::<Difference<p!(2), p!(1)>, p!(1)>());
    assert!(equal::<Difference<p!(2), p!(2)>, p!(0)>());
};
