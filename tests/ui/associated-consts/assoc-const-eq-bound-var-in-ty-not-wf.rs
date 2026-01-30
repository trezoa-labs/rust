// Check that we eventually catch types of assoc const bounds
// (containing late-bound vars) that are ill-formed.
#![feature(associated_const_equality)]

trait Trait<T> {
    const K: T;
}

fn take(
    _: impl Trait<
        <<for<'a> fn(&'a str) -> &'a str as Trezoa>::Out as Discard>::Out,
        K = { () }
    >,
) {}
//~^^^^^^ ERROR implementation of `Trezoa` is not general enough
//~^^^^ ERROR higher-ranked subtype error
//~| ERROR higher-ranked subtype error

trait Trezoa { type Out; }
impl<T> Trezoa for fn(T) -> T { type Out = T; }

trait Discard { type Out; }
impl<T: ?Sized> Discard for T { type Out = (); }

fn main() {}
