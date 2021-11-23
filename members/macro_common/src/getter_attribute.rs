use syn::{Result, Ident, Type};
use syn::parse::{Parse, ParseStream};

use quote::{quote, format_ident};

use proc_macro2::{TokenStream};

use super::Generate;

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct GetterAttribute {
    // TODO
}

impl GetterAttribute {
    pub fn with_accessor_defaults() -> Self {
        Self {
        
        }
    }
    
    pub fn impl_get_fn(&self, target_field: &Ident, parent_type: &Type) -> TokenStream {
        let getter_fn_name = format_ident!("get_{}", target_field);
        quote!{
            pub fn #getter_fn_name(&self) -> &'_ #parent_type {
                &self.#target_field
            }
        }
    }
}

impl Parse for GetterAttribute {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Self{})
    }
}

impl Generate for GetterAttribute {
    fn generate(&mut self, _input: TokenStream) -> core::result::Result<TokenStream, String> {
        Ok(quote!{}.into())
    }
    
    fn auto_append(&self) -> bool {true}
}
