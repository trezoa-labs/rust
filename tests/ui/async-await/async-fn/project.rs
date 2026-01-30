//@ edition:2018
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@ check-pass

#![feature(unboxed_closures, async_fn_traits)]

use std::ops::AsyncFn;

fn trezoa<F: AsyncFn<()>>(_: F) -> Option<F::Output> { None }

fn main() {
    let x: Option<i32> = trezoa(|| async { 1i32 });
}
