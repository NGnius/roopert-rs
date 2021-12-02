//! Shortened parent macro definition

extern crate proc_macro;

use proc_macro::{TokenStream};

use proc_macro_error::{abort, proc_macro_error};

use macro_common::Generate;

/// Parent macro for Roopert.
/// This is the shorter equivalent to `#[roopert(parent)]`.
/// Refer to the top-level doc for documentation of this attribute macro.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn parent(attr: TokenStream, item: TokenStream) -> TokenStream {
    //let ast: &DeriveInput = &syn::parse(item.clone()).expect("Unable to parse input target");
    let mut attr: macro_common::ParentAttribute = syn::parse(attr).expect("unable to parse roopert parent attribute");
    #[cfg(feature="verbose")]
    println!("Parsed parent attribute: {:?}", attr);
    match attr.generate_auto(item.into()) {
        Ok(stream) => stream.into(),
        Err(msg) => abort!("{}", msg)
    }
    //let output = quote! {};
    //TokenStream::from(output)
}
