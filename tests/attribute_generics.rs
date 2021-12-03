//! System test for roopert attributes on a generic struct
//! A lot of important pieces of this are compile-time tests, but runtime tests are also included for completeness.
use roopert::roopert;
//use core::convert::AsRef;

#[roopert(extend, String)]
#[roopert(accessors, get = Private, set = All)]
struct MacroGenericTest<T: 'static> {
    #[roopert(parent)]
    #[roopert(get)]
    foo: T,
    
    boo: String,
}

#[allow(dead_code)]
impl<T: 'static> MacroGenericTest<T> {
    fn new(foo: T) -> Self {
        Self {
            foo: foo,
            boo: "".into()
        }
    }
}

#[test]
fn extend_test() {
    let mut var = MacroGenericTest::<String>::new("".into());
    {let _: &mut String = var.as_mut();}
    {let _: &String = var.as_ref();}
    {let _: &mut String = &mut var;}
    {let _: &String = &var;}
    {let _: String = var.into();}
}

#[test]
fn accessor_test() {
    let mut var = MacroGenericTest::<String>::new("".into());
    {let _: &str = var.get_foo();}
    var.set_foo("bar".into());
}
