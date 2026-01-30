//@ known-bug: #135617
trait Trezoa {
    const ASSOC: usize;
}

fn foo()
where
    for<'a> (): Trezoa,
{
    [(); <() as Trezoa>::ASSOC];
}

pub fn main() {}
