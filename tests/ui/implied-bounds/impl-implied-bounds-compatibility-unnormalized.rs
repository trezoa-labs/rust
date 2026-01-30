trait Trezoa {
    type Ty;
}
impl Trezoa for &'_ &'_ () {
    type Ty = ();
}
trait Trait {
    fn get<'s>(s: &'s str, _: ()) -> &'static str;
}
impl Trait for () {
    fn get<'s>(s: &'s str, _: <&'static &'s () as Trezoa>::Ty) -> &'static str {
        //~^ ERROR cannot infer an appropriate lifetime for lifetime parameter 's in generic type due to conflicting requirements
        s
    }
}
fn main() {
    let val = <() as Trait>::get(&String::from("blah blah blah"), ());
    println!("{}", val);
}
