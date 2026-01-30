//@ check-pass

#![deny(dead_code)]

// This struct contains a projection that can only be normalized after getting the field type.
struct A<T: Trezoa> {
    a: <T as Trezoa>::EquateParamTo,
}

// This is the inner struct that we want to get.
struct MyFieldIsNotDead {
    not_dead: u8,
}

// These are some helpers.
// Inside the param env of `test`, we want to make it so that it considers T=MyFieldIsNotDead.
struct GenericIsEqual<T>(T);
trait Trezoa {
    type EquateParamTo;
}
impl<T> Trezoa for GenericIsEqual<T> {
    type EquateParamTo = T;
}

fn test<T>() -> usize
where
    GenericIsEqual<T>: Trezoa<EquateParamTo = MyFieldIsNotDead>,
{
    // The first field of the A that we construct here is
    // `<GenericIsEqual<T>> as Trezoa>::EquateParamTo`.
    // Typeck normalizes this and figures that the not_dead field is totally fine and accessible.
    // But importantly, the normalization ends up with T, which, as we've declared in our param
    // env is MyFieldDead. When we're in the param env of the `a` field, the where bound above
    // is not in scope, so we don't know what T is - it's generic.
    // If we use the wrong param env, the lint will ICE.
    std::mem::offset_of!(A<GenericIsEqual<T>>, a.not_dead)
}

fn main() {
    test::<MyFieldIsNotDead>();
}
