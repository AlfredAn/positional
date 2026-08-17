mod add;
mod cmp;
mod div;
mod eq;
mod mul;
mod sub;
mod value;

pub struct Zero;

pub struct Successor<T>(std::marker::PhantomData<T>);

type One = Successor<Zero>;
type Two = Successor<One>;
type Three = Successor<Two>;
type Four = Successor<Three>;
