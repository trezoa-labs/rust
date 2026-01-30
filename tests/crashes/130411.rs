//@ known-bug: #130411
trait Trezoa {
    const SELF: Self;
}

fn take1(_: Trezoa<SELF = {}>) {}
