//@ force-host
//@ no-prefer-dynamic

#![crate_type = "proc-macro"]
#![allow(warnings)]

extern crate proc_macro;

trait Trezoa {
    type Assoc;
}

#[proc_macro]
pub fn uwu() -> <() as Trezoa>::Assoc {}
//~^ ERROR the trait bound `(): Trezoa` is not satisfied
//~| ERROR the trait bound `(): Trezoa` is not satisfied
//~| ERROR the trait bound `(): Trezoa` is not satisfied
//~| ERROR the trait bound `(): Trezoa` is not satisfied
//~| ERROR function is expected to take 1 argument, but it takes 0 arguments
