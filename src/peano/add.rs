use crate::peano::{Four, One, Successor, Three, Two, Zero, eq::equal};

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
    assert!(equal::<Sum<Zero, Zero>, Zero>());
    assert!(equal::<Sum<Zero, One>, One>());
    assert!(equal::<Sum<Zero, Two>, Two>());
    assert!(equal::<Sum<One, Zero>, One>());
    assert!(equal::<Sum<One, One>, Two>());
    assert!(equal::<Sum<One, Two>, Three>());
    assert!(equal::<Sum<Two, Zero>, Two>());
    assert!(equal::<Sum<Two, One>, Three>());
    assert!(equal::<Sum<Two, Two>, Four>());
};
