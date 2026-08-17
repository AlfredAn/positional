pub trait Bool {
    const VALUE: bool;
}

pub struct False;
pub struct True;

impl Bool for False {
    const VALUE: bool = false;
}

impl Bool for True {
    const VALUE: bool = true;
}

pub trait And<T> {
    type Result: Bool;
}

pub type Both<T1, T2> = <T1 as And<T2>>::Result;

impl And<False> for False {
    type Result = False;
}

impl And<True> for False {
    type Result = False;
}

impl And<False> for True {
    type Result = False;
}

impl And<True> for True {
    type Result = True;
}
