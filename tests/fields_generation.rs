//! Field metadata tests

//use std::any::Any;

use roopert::{roopert, Fields};

#[roopert(fields, rule = all)]
#[derive(Default)]
struct RoopertFieldsTest {
    field1: String,
    #[field(meta = "42")]
    field2: u32,
}

#[test]
fn fields_test() {
    let mut var = RoopertFieldsTest::default();
    // field1
    assert!(var.get("field1").unwrap().is::<String>());
    assert_eq!(var.get("field1").unwrap().downcast_ref::<String>().unwrap(), &String::default());
    assert_eq!(var.get("field1").unwrap().downcast_ref::<String>().unwrap(), &var.field1);
    // field2
    assert_eq!(var.get("field2").unwrap().downcast_ref::<u32>().unwrap(), &u32::default());
    assert_eq!(var.get("field2").unwrap().downcast_ref::<u32>().unwrap(), &var.field2);
    *(var.get_mut("field2").unwrap().downcast_mut::<u32>().unwrap()) = 42;
    assert_eq!(var.get("field2").unwrap().downcast_ref::<u32>().unwrap(), &42_u32);
    assert_eq!(var.metadata("field2").unwrap(), "42");
}
