//! Roopert root macro definition

//extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro_error::{abort, proc_macro_error};

use roopert_macro_common::{RoopertAttribute, Generate};

/// Root macro for Roopert.
/// All attribute macros are of the form `#[roopert(type)]`,
/// where `type` is the type of attribute.
/// Refer to the top-level doc for supported types.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn roopert(attr: TokenStream, item: TokenStream) -> TokenStream {
    //let ast: &DeriveInput = &syn::parse(item.clone()).expect("Unable to parse input target");
    let mut attr: RoopertAttribute = syn::parse(attr).expect("Unable to parse roopert attribute");
    #[cfg(feature="verbose")]
    println!("Parsed roopert attribute: {:?}", attr);
    match attr.generate_auto(item.into()) {
        Ok(stream) => stream.into(),
        Err(msg) => abort!("{}", msg)
    }
    //let output = quote! {};
    //TokenStream::from(output)
}
