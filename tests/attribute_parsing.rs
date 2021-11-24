//! System test for roopert attribute
//! A lot of important pieces of this are compile-time tests, but runtime tests are also included for completeness.
use roopert::roopert;
//use core::convert::AsRef;

#[roopert(extend, String)]
#[roopert(accessors, get = Private, set = All)]
#[derive(Default)]
struct MacroRootTest {
    #[roopert(parent)]
    #[roopert(get)]
    foo: String,
}

#[allow(dead_code)]
impl MacroRootTest {
    fn new() -> Self {
        Self {
            foo: "".into(),
        }
    }
}

#[test]
fn extend_test() {
    let mut var = MacroRootTest::default();
    {let _: &mut String = var.as_mut();}
    {let _: &String = var.as_ref();}
    {let _: &mut String = &mut var;}
    {let _: &String = &var;}
    {let _: String = var.into();}
}

#[test]
fn accessor_test() {
    let mut var = MacroRootTest::default();
    {let _: &str = var.get_foo();}
    var.set_foo("bar".into());
}
