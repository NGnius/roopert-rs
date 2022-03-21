use std::any::Any;

pub trait Fields {
    fn fields(&self) -> &[&str];
    
    fn get(&self, field_name: &str) -> Option<&'_ dyn Any>;
    
    fn get_mut(&mut self, field_name: &str) -> Option<&'_ mut dyn Any>;
    
    fn metadata(&self, field_name: &str) -> Option<&str>;
}
