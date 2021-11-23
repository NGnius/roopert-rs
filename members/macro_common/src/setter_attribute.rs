use syn::{Result, Ident, Type};
use syn::parse::{Parse, ParseStream};

use quote::{quote, format_ident};

use proc_macro2::{TokenStream};

use super::Generate;

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct SetterAttribute {
    // TODO
}

impl SetterAttribute {
    pub fn with_accessor_defaults() -> Self {
        Self {
        
        }
    }
    
    pub fn impl_set_fn(&self, target_field: &Ident, parent_type: &Type) -> TokenStream {
        let setter_fn_name = format_ident!("set_{}", target_field);
        quote!{
            pub fn #setter_fn_name(&mut self, x: #parent_type) {
                self.#target_field = x;
            }
        }
    }
}

impl Parse for SetterAttribute {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Self{})
    }
}

impl Generate for SetterAttribute {
    fn generate(&mut self, _input: TokenStream) -> core::result::Result<TokenStream, String> {
        Ok(quote!{}.into())
    }
    
    fn auto_append(&self) -> bool {true}
}
