use std::marker::PhantomData;

trait Bool {
    const VALUE: bool;
}

struct False;
struct True;

impl Bool for False {
    const VALUE: bool = false;
}

impl Bool for True {
    const VALUE: bool = true;
}

struct Zero;

struct Successor<T>(PhantomData<T>);

type One = Successor<Zero>;
type Two = Successor<One>;
type Three = Successor<Two>;
type Four = Successor<Three>;

trait Value {
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

trait Equal<T> {
    type Equal: Bool;
}

impl Equal<Zero> for Zero {
    type Equal = True;
}

impl<T> Equal<Successor<T>> for Zero {
    type Equal = False;
}

impl<T> Equal<Zero> for Successor<T> {
    type Equal = False;
}

impl<T1, T2> Equal<Successor<T2>> for Successor<T1>
where
    T1: Equal<T2>,
{
    type Equal = <T1 as Equal<T2>>::Equal;
}

const fn equal<T1, T2>() -> bool
where
    T1: Equal<T2>,
{
    <T1 as Equal<T2>>::Equal::VALUE
}

trait Add<T> {
    type Sum;
}

type Sum<T1, T2> = <T1 as Add<T2>>::Sum;

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

trait Subtract<T> {
    type Difference;
}

type Difference<T1, T2> = <T1 as Subtract<T2>>::Difference;

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

trait Multiply<T> {
    type Product;
}

type Product<T1, T2> = <T1 as Multiply<T2>>::Product;

impl Multiply<Zero> for Zero {
    type Product = Zero;
}

impl<T> Multiply<Successor<T>> for Zero {
    type Product = Zero;
}

impl<T> Multiply<Zero> for Successor<T> {
    type Product = Zero;
}

impl<T1, T2> Multiply<Successor<T2>> for Successor<T1>
where
    T1: Multiply<Successor<T2>>,
    Product<T1, Successor<T2>>: Add<Successor<T2>>,
{
    type Product = Sum<Product<T1, Successor<T2>>, Successor<T2>>;
}

trait Lt<T> {
    type Lt: Bool;
}

impl Lt<Zero> for Zero {
    type Lt = False;
}

impl<T> Lt<Successor<T>> for Zero {
    type Lt = True;
}

impl<T> Lt<Zero> for Successor<T> {
    type Lt = False;
}

impl<T1, T2> Lt<Successor<T2>> for Successor<T1>
where
    T1: Lt<T2>,
{
    type Lt = <T1 as Lt<T2>>::Lt;
}

const fn lt<T1, T2>() -> bool
where
    T1: Lt<T2>,
{
    <T1 as Lt<T2>>::Lt::VALUE
}

trait NotLt<T>: Lt<T, Lt = False> {}
trait IsLt<T>: Lt<T, Lt = True> {}

impl<T1, T2> NotLt<T2> for T1 where T1: Lt<T2, Lt = False> {}
impl<T1, T2> IsLt<T2> for T1 where T1: Lt<T2, Lt = True> {}

mod div {
    use super::*;

    pub(super) trait Helper<B> {
        type Quotient;
        type Remainder;
    }

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
}

trait Divide<T> {
    type Quotient;
    type Remainder;
}

type Quotient<T1, T2> = <T1 as Divide<T2>>::Quotient;
type Remainder<T1, T2> = <T1 as Divide<T2>>::Remainder;

impl<T1, T2> Divide<Successor<T2>> for T1
where
    T1: Lt<Successor<T2>>,
    (T1, Successor<T2>): div::Helper<<T1 as Lt<Successor<T2>>>::Lt>,
{
    type Quotient = <(T1, Successor<T2>) as div::Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Quotient;
    type Remainder = <(T1, Successor<T2>) as div::Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Remainder;
}

#[expect(unused)]
const STATIC_ASSERTS: () = static_asserts();

const fn static_asserts() {
    assert!(Zero::VALUE == 0);
    assert!(One::VALUE == 1);
    assert!(Two::VALUE == 2);
    assert!(Three::VALUE == 3);
    assert!(Four::VALUE == 4);

    assert!(equal::<Zero, Zero>());
    assert!(!equal::<Zero, One>());
    assert!(!equal::<Zero, Two>());
    assert!(!equal::<One, Zero>());
    assert!(equal::<One, One>());
    assert!(!equal::<One, Two>());
    assert!(!equal::<Two, Zero>());
    assert!(!equal::<Two, One>());
    assert!(equal::<Two, Two>());

    assert!(equal::<Sum<Zero, Zero>, Zero>());
    assert!(equal::<Sum<Zero, One>, One>());
    assert!(equal::<Sum<Zero, Two>, Two>());
    assert!(equal::<Sum<One, Zero>, One>());
    assert!(equal::<Sum<One, One>, Two>());
    assert!(equal::<Sum<One, Two>, Three>());
    assert!(equal::<Sum<Two, Zero>, Two>());
    assert!(equal::<Sum<Two, One>, Three>());
    assert!(equal::<Sum<Two, Two>, Four>());

    assert!(equal::<Difference<Zero, Zero>, Zero>());
    assert!(equal::<Difference<One, Zero>, One>());
    assert!(equal::<Difference<One, One>, Zero>());
    assert!(equal::<Difference<Two, Zero>, Two>());
    assert!(equal::<Difference<Two, One>, One>());
    assert!(equal::<Difference<Two, Two>, Zero>());

    assert!(equal::<Product<Zero, Zero>, Zero>());
    assert!(equal::<Product<Zero, One>, Zero>());
    assert!(equal::<Product<Zero, Two>, Zero>());
    assert!(equal::<Product<One, Zero>, Zero>());
    assert!(equal::<Product<One, One>, One>());
    assert!(equal::<Product<One, Two>, Two>());
    assert!(equal::<Product<Two, Zero>, Zero>());
    assert!(equal::<Product<Two, One>, Two>());
    assert!(equal::<Product<Two, Two>, Four>());

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

    assert!(!lt::<Zero, Zero>());
    assert!(lt::<Zero, One>());
    assert!(lt::<Zero, Two>());
    assert!(!lt::<One, Zero>());
    assert!(!lt::<One, One>());
    assert!(lt::<One, Two>());
    assert!(!lt::<Two, Zero>());
    assert!(!lt::<Two, One>());
    assert!(!lt::<Two, Two>());
}
