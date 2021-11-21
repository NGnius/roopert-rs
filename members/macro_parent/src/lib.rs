extern crate proc_macro;

use proc_macro::{TokenStream};

use macro_common::Generate;

#[proc_macro_attribute]
pub fn parent(attr: TokenStream, item: TokenStream) -> TokenStream {
    //let ast: &DeriveInput = &syn::parse(item.clone()).expect("Unable to parse input target");
    let mut attr: macro_common::ParentAttribute = syn::parse(attr).expect("unable to parse roopert parent attribute");
    #[cfg(feature="verbose")]
    println!("Parsed parent attribute: {:?}", attr);
    match attr.generate_auto(item.into()) {
        Ok(stream) => stream.into(),
        Err(msg) => panic!("{}", msg)
    }
    //let output = quote! {};
    //TokenStream::from(output)
}
