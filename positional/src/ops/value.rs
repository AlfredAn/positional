use crate::prelude::*;

pub trait Value {
    const VALUE: u64;
}

impl Value for PeanoZero {
    const VALUE: u64 = 0;
}

impl<T> Value for PeanoSucc<T>
where
    T: Value,
{
    const VALUE: u64 = T::VALUE + 1;
}

impl<R> Value for Term<R> {
    const VALUE: u64 = 0;
}

impl<R, H, T> Value for Seq<R, H, T>
where
    R: Value,
    H: Value,
    T: Value,
{
    const VALUE: u64 = H::VALUE + R::VALUE * T::VALUE;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
    assert!(<p!(0)>::VALUE == 0);
    assert!(<p!(1)>::VALUE == 1);
    assert!(<p!(2)>::VALUE == 2);
    assert!(<p!(3)>::VALUE == 3);
    assert!(<p!(4)>::VALUE == 4);
    assert!(<p!(1000)>::VALUE == 1000);

    define_encoding!(binary, "01");
    assert!(<number!(binary, "0")>::VALUE == 0b0);
    assert!(<number!(binary, "1")>::VALUE == 0b1);
    assert!(<number!(binary, "10")>::VALUE == 0b10);
    assert!(<number!(binary, "11")>::VALUE == 0b11);
    assert!(<number!(binary, "1101011")>::VALUE == 0b1101011);
    assert!(<number!(binary, "01101011")>::VALUE == 0b1101011);

    define_encoding!(hex, "0123456789abcdef");
    assert!(<number!(hex, "deadbeef")>::VALUE == 0xdeadbeef);
};
