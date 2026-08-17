use crate::prelude::*;

pub trait Subtract<T> {
    type Difference;
}

pub type Difference<T1, T2> = <T1 as Subtract<T2>>::Difference;

impl Subtract<Zero> for Zero {
    type Difference = Zero;
}

impl<T> Subtract<Zero> for Successor<T> {
    type Difference = Successor<T>;
}

impl<T1, T2> Subtract<Successor<T2>> for Successor<T1>
where
    T1: Subtract<T2>,
{
    type Difference = Difference<T1, T2>;
}

const _: () = const {
    use positional_macro::peano as p;
    assert!(equal::<Difference<p!(0), p!(0)>, p!(0)>());
    assert!(equal::<Difference<p!(1), p!(0)>, p!(1)>());
    assert!(equal::<Difference<p!(1), p!(1)>, p!(0)>());
    assert!(equal::<Difference<p!(2), p!(0)>, p!(2)>());
    assert!(equal::<Difference<p!(2), p!(1)>, p!(1)>());
    assert!(equal::<Difference<p!(2), p!(2)>, p!(0)>());
};
