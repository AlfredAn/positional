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

impl<R, T> Add<T> for Term<R>
where
    R: Radix,
    Term<R>: CarryAdd<T, PeanoZero>,
{
    type Sum = CarrySum<Term<R>, T, PeanoZero>;
}

impl<R, H1, T1, T2> Add<T2> for Seq<R, H1, T1>
where
    R: Radix,
    Seq<R, H1, T1>: CarryAdd<T2, PeanoZero>,
{
    type Sum = CarrySum<Seq<R, H1, T1>, T2, PeanoZero>;
}

pub trait CarryAdd<T, C> {
    type Sum;
}

pub type CarrySum<T1, T2, C> = <T1 as CarryAdd<T2, C>>::Sum;

impl<R> CarryAdd<Term<R>, PeanoZero> for Term<R> {
    type Sum = Term<R>;
}

impl<R, H, T> CarryAdd<Seq<R, H, T>, PeanoZero> for Term<R> {
    type Sum = Seq<R, H, T>;
}

impl<R, H, T> CarryAdd<Term<R>, PeanoZero> for Seq<R, H, T> {
    type Sum = Seq<R, H, T>;
}

impl<R, C> CarryAdd<Term<R>, PeanoSucc<C>> for Term<R>
where
    PeanoSucc<C>: Divide<R>,
    Term<R>: CarryAdd<Term<R>, Quotient<PeanoSucc<C>, R>>,
{
    // H0 = C % R
    // C0 = C / R
    // T0 = C0
    type Sum =
        Seq<R, Remainder<PeanoSucc<C>, R>, CarrySum<Term<R>, Term<R>, Quotient<PeanoSucc<C>, R>>>;
}

impl<R, H, T, C> CarryAdd<Seq<R, H, T>, PeanoSucc<C>> for Term<R>
where
    PeanoSucc<C>: Add<H>,
    Sum<PeanoSucc<C>, H>: Divide<R>,
    Term<R>: CarryAdd<T, Quotient<Sum<PeanoSucc<C>, H>, R>>,
{
    // H0 = (C + H) % R
    // C0 = (C + H) / R
    // T0 = 0 + T + C0
    type Sum = Seq<
        R,
        Remainder<Sum<PeanoSucc<C>, H>, R>,
        CarrySum<Term<R>, T, Quotient<Sum<PeanoSucc<C>, H>, R>>,
    >;
}

impl<R, H, T, C> CarryAdd<Term<R>, PeanoSucc<C>> for Seq<R, H, T>
where
    PeanoSucc<C>: Add<H>,
    Sum<PeanoSucc<C>, H>: Divide<R>,
    T: CarryAdd<Term<R>, Quotient<Sum<PeanoSucc<C>, H>, R>>,
{
    // H0 = (C + H) % R
    // C0 = (C + H) / R
    // T0 = T + 0 + C0
    type Sum = Seq<
        R,
        Remainder<Sum<PeanoSucc<C>, H>, R>,
        CarrySum<T, Term<R>, Quotient<Sum<PeanoSucc<C>, H>, R>>,
    >;
}

impl<R, H1, H2, T1, T2, C> CarryAdd<Seq<R, H2, T2>, C> for Seq<R, H1, T1>
where
    H1: Add<H2>,
    C: Add<Sum<H1, H2>>,
    Sum<C, Sum<H1, H2>>: Divide<R>,
    T1: CarryAdd<T2, Quotient<Sum<C, Sum<H1, H2>>, R>>,
{
    // H0 = (C + H1 + H2) % R
    // C0 = (C + H1 + H2) / R
    // T0 = T1 + T2 + C0
    type Sum = Seq<
        R,
        Remainder<Sum<C, Sum<H1, H2>>, R>,
        CarrySum<T1, T2, Quotient<Sum<C, Sum<H1, H2>>, R>>,
    >;
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

    define_encoding!(bin, "01");
    assert!(equal::<
        Sum<number!(bin, "0"), number!(bin, "0")>,
        number!(bin, "0"),
    >());
    assert!(equal::<
        Sum<number!(bin, "0"), number!(bin, "1")>,
        number!(bin, "1"),
    >());
    assert!(equal::<
        Sum<number!(bin, "1"), number!(bin, "1")>,
        number!(bin, "10"),
    >());

    type LongBin0 = number!(
        bin,
        "1101111010101101101111101110111100000001001000110100110010101111111010001001000000010010001101000101"
    );
    type LongBin1 = number!(
        bin,
        "11001100101111011011111011101111110111100101100010111110011100000010110110101111111010001001000010111100010010000110"
    );
    type ExpectedBin = number!(
        bin,
        "11001100101111101001110110011101100111010100011110111111100100110111101001011111110100010010000011001110011111001011"
    );
    assert!(equal::<Sum<LongBin0, LongBin1>, ExpectedBin>());

    define_encoding!(hex, "0123456789ABCDEF");
    type LongHex0 = number!(hex, "DEADBEEF01234CAFE89012345");
    type LongHex1 = number!(hex, "CCBDBEEFDE58BE702DAFE890BC486");
    type ExpectedHex = number!(hex, "CCBE9D9D9D47BF937A5FD120CE7CB");
    assert!(equal::<Sum<LongHex0, LongHex1>, ExpectedHex>());
};
