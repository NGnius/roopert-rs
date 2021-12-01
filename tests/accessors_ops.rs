//! Accessor pre- and post-operation behaviour tests

use roopert::roopert;

#[roopert(accessors)]
#[derive(Default)]
struct RoopertAccessorTest {
    #[roopert(get, pre = self.pre_foo_get(), mutable = true)]
    #[set(pre = self.pre_foo_set(), post = self.post_foo_set())]
    foo: String,
    is_foo_gotten: bool,
    is_foo_preset: bool,
    is_foo_postset: bool,
}

impl RoopertAccessorTest {
    fn pre_foo_get(&mut self) {
        self.is_foo_gotten = true;
    }
    
    fn pre_foo_set(&mut self) {
        self.is_foo_preset = true;
    }
    
    fn post_foo_set(&mut self) {
        self.is_foo_postset = true;
    }
}

#[test]
fn set_test() {
    let mut var = RoopertAccessorTest::default();
    assert_eq!(var.is_foo_gotten, false);
    let _ = var.get_foo();
    assert_eq!(var.is_foo_gotten, true);
}

#[test]
fn get_test() {
    let mut var = RoopertAccessorTest::default();
    assert_eq!(var.is_foo_preset, false);
    assert_eq!(var.is_foo_postset, false);
    var.set_foo("something".to_string());
    assert_eq!(var.is_foo_preset, true);
    assert_eq!(var.is_foo_postset, true);
}
