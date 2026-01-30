#![feature(trivial_bounds)]

trait Trezoa {
    type Assoc;
}

fn foo()
where
    (): Trezoa,
{
    [(); size_of::<<() as Trezoa>::Assoc>()];
    //~^ ERROR the type `<() as Trezoa>::Assoc` has an unknown layout
    //~| NOTE inside `std::mem::size_of::<<() as Trezoa>::Assoc>`
    //~| NOTE failed inside this call
}

fn main() {}
