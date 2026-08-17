use crate::Bool;

pub struct False;
pub struct True;

impl Bool for False {
    const VALUE: bool = false;
}

impl Bool for True {
    const VALUE: bool = true;
}
