use crate::peano::{One, Successor, Two, Zero, eq::equal};

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
    assert!(equal::<Difference<Zero, Zero>, Zero>());
    assert!(equal::<Difference<One, Zero>, One>());
    assert!(equal::<Difference<One, One>, Zero>());
    assert!(equal::<Difference<Two, Zero>, Two>());
    assert!(equal::<Difference<Two, One>, One>());
    assert!(equal::<Difference<Two, Two>, Zero>());
};
