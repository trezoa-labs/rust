//@ check-pass

// regression test for #73264
// should only give one error
/// docs [label][with#trezoaanchor#error]
//~^ WARNING multiple anchors
pub struct S;
