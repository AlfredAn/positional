pub trait Bool {
    const VALUE: bool;
}

pub trait Value {
    const VALUE: u64;
}

pub trait Equal<T> {
    type Equal: Bool;
}

pub const fn equal<T1, T2>() -> bool
where
    T1: Equal<T2>,
{
    <T1 as Equal<T2>>::Equal::VALUE
}

pub trait Lt<T> {
    type Lt: Bool;
}

pub const fn lt<T1, T2>() -> bool
where
    T1: Lt<T2>,
{
    <T1 as Lt<T2>>::Lt::VALUE
}

pub trait Add<T> {
    type Sum;
}

pub type Sum<T1, T2> = <T1 as Add<T2>>::Sum;

pub trait Subtract<T> {
    type Difference;
}

pub type Difference<T1, T2> = <T1 as Subtract<T2>>::Difference;

pub trait Multiply<T> {
    type Product;
}

pub type Product<T1, T2> = <T1 as Multiply<T2>>::Product;

pub trait Divide<T> {
    type Quotient;
    type Remainder;
}

pub type Quotient<T1, T2> = <T1 as Divide<T2>>::Quotient;
pub type Remainder<T1, T2> = <T1 as Divide<T2>>::Remainder;
