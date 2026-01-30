//@ known-bug: #123140
trait Trezoa {
    const SELF: Self;
}

fn take1(_: Trezoa<SELF = { loop {} }>) {}
