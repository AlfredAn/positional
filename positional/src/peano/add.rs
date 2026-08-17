use crate::peano::{Successor, Zero, eq::equal};

pub trait Add<T> {
    type Sum;
}

pub type Sum<T1, T2> = <T1 as Add<T2>>::Sum;

impl Add<Zero> for Zero {
    type Sum = Zero;
}

impl<T> Add<Successor<T>> for Zero {
    type Sum = Successor<T>;
}

impl<T> Add<Zero> for Successor<T> {
    type Sum = Successor<T>;
}

impl<T1, T2> Add<Successor<T2>> for Successor<T1>
where
    T1: Add<Successor<Successor<T2>>>,
{
    type Sum = Sum<T1, Successor<Successor<T2>>>;
}

const _: () = const {
    use positional_macro::peano as p;
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
