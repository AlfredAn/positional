use crate::peano::{Four, One, Successor, Three, Two, Zero};

pub trait Value {
    const VALUE: u64;
}

impl Value for Zero {
    const VALUE: u64 = 0;
}

impl<T> Value for Successor<T>
where
    T: Value,
{
    const VALUE: u64 = T::VALUE + 1;
}

const _: () = const {
    assert!(Zero::VALUE == 0);
    assert!(One::VALUE == 1);
    assert!(Two::VALUE == 2);
    assert!(Three::VALUE == 3);
    assert!(Four::VALUE == 4);
};
