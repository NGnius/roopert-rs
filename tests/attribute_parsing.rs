use roopert::roopert;
//use core::convert::AsRef;

#[roopert(extend, String)]
#[derive(Default)]
struct MacroRootTest {
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
fn build_test() {
    let mut var = MacroRootTest::default();
    {let _: &mut String = var.as_mut();}
    {let _: &String = var.as_ref();}
    {let _: String = var.into();}
}
