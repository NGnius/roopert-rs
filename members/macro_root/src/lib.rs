//extern crate proc_macro;

use proc_macro::TokenStream;

use macro_common::{RoopertAttribute, Generate};

#[proc_macro_attribute]
pub fn roopert(attr: TokenStream, item: TokenStream) -> TokenStream {
    //let ast: &DeriveInput = &syn::parse(item.clone()).expect("Unable to parse input target");
    let mut attr: RoopertAttribute = syn::parse(attr).expect("Unable to parse roopert attribute");
    #[cfg(feature="verbose")]
    println!("Parsed roopert attribute: {:?}", attr);
    match attr.generate_auto(item.into()) {
        Ok(stream) => stream.into(),
        Err(msg) => panic!("{}", msg)
    }
    //let output = quote! {};
    //TokenStream::from(output)
}
