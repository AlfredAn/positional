use crate::{
    bool::{False, True},
    peano::{
        Four, One, Successor, Three, Two, Zero,
        cmp::{IsLt, Lt, NotLt},
        eq::equal,
        sub::{Difference, Subtract},
    },
};

pub trait Divide<T> {
    type Quotient;
    type Remainder;
}

pub type Quotient<T1, T2> = <T1 as Divide<T2>>::Quotient;
pub type Remainder<T1, T2> = <T1 as Divide<T2>>::Remainder;

impl<T1, T2> Divide<Successor<T2>> for T1
where
    T1: Lt<Successor<T2>>,
    (T1, Successor<T2>): Helper<<T1 as Lt<Successor<T2>>>::Lt>,
{
    type Quotient = <(T1, Successor<T2>) as Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Quotient;
    type Remainder = <(T1, Successor<T2>) as Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Remainder;
}

mod hidden {
    pub trait Helper<B> {
        type Quotient;
        type Remainder;
    }
}

use hidden::Helper;

impl<T1, T2> Helper<True> for (T1, T2)
where
    T1: IsLt<T2>,
{
    type Quotient = Zero;
    type Remainder = T1;
}

impl<T1, T2> Helper<False> for (T1, T2)
where
    T1: NotLt<T2>,
    T1: Subtract<T2>,
    Difference<T1, T2>: Divide<T2>,
{
    type Quotient = Successor<Quotient<Difference<T1, T2>, T2>>;
    type Remainder = Remainder<Difference<T1, T2>, T2>;
}

const _: () = const {
    assert!(equal::<Quotient<Zero, One>, Zero>());
    assert!(equal::<Quotient<Zero, Two>, Zero>());
    assert!(equal::<Quotient<One, One>, One>());
    assert!(equal::<Quotient<One, Two>, Zero>());
    assert!(equal::<Quotient<Two, One>, Two>());
    assert!(equal::<Quotient<Two, Two>, One>());
    assert!(equal::<Quotient<Three, One>, Three>());
    assert!(equal::<Quotient<Three, Two>, One>());
    assert!(equal::<Quotient<Four, One>, Four>());
    assert!(equal::<Quotient<Four, Two>, Two>());

    assert!(equal::<Remainder<Zero, One>, Zero>());
    assert!(equal::<Remainder<Zero, Two>, Zero>());
    assert!(equal::<Remainder<One, One>, Zero>());
    assert!(equal::<Remainder<One, Two>, One>());
    assert!(equal::<Remainder<Two, One>, Zero>());
    assert!(equal::<Remainder<Two, Two>, Zero>());
    assert!(equal::<Remainder<Three, One>, Zero>());
    assert!(equal::<Remainder<Three, Two>, One>());
    assert!(equal::<Remainder<Four, One>, Zero>());
    assert!(equal::<Remainder<Four, Two>, Zero>());
};
