use syn::{Result};
use syn::parse::{Parse, ParseStream};

use quote::quote;

use proc_macro2::{TokenStream};

use super::{Generate};

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct ParentAttribute {
}

impl Parse for ParentAttribute {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Self{})
    }
}

impl Generate for ParentAttribute {
    fn generate(&mut self, _input: TokenStream) -> core::result::Result<TokenStream, String> {
        Ok(quote!{}.into())
    }
    
    fn auto_append(&self) -> bool {true}
}
