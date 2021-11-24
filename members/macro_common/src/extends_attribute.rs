//use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

use proc_macro2::{TokenStream};

use syn::{ItemStruct, Ident, Result, Token, punctuated::Punctuated, Type};
use syn::parse::{Parse, ParseStream};

use quote::quote;

use super::{Generate, ParentAttribute, RoopertAttribute, RoopertAttributeType};

use super::parse::{is_parent_attribute, is_roopert_attribute};

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct ExtendsAttribute {
    types: Punctuated<Type, Token![,]>,
}

impl ExtendsAttribute {
    fn impl_asref(target_struct_ident: &Ident, target_field: &Ident, parent_type: &Type) -> TokenStream {
        quote!{
            impl core::convert::AsRef<#parent_type> for #target_struct_ident {
                fn as_ref(&self) -> &#parent_type {
                    &self.#target_field
                }
            }
        }
    }
    
    fn impl_asmut(target_struct_ident: &Ident, target_field: &Ident, parent_type: &Type) -> TokenStream {
        quote!{
            impl core::convert::AsMut<#parent_type> for #target_struct_ident {
                fn as_mut(&mut self) -> &mut #parent_type {
                    &mut self.#target_field
                }
            }
        }
    }
    
    fn impl_into(target_struct_ident: &Ident, target_field: &Ident, parent_type: &Type) -> TokenStream {
        quote!{
            impl core::convert::Into<#parent_type> for #target_struct_ident {
                fn into(self) -> #parent_type {
                    self.#target_field
                }
            }
        }
    }
    
    fn impl_deref(target_struct_ident: &Ident, target_field: &Ident, parent_type: &Type) -> TokenStream {
        quote!{
            impl core::ops::Deref for #target_struct_ident {
                type Target = #parent_type;
                fn deref(&self) -> &Self::Target {
                    &self.#target_field
                }
            }
        }
    }
    
    fn impl_derefmut(target_struct_ident: &Ident, target_field: &Ident, _parent_type: &Type) -> TokenStream {
        quote!{
            impl core::ops::DerefMut for #target_struct_ident {
                // type Target = #parent_type; // (inferred by Deref impl)
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#target_field
                }
            }
        }
    }
}

impl Parse for ExtendsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            types: Punctuated::<Type, Token![,]>::parse_separated_nonempty(input)?,
        })
    }
}

impl Generate for ExtendsAttribute {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        // parse input
        let mut target_struct: ItemStruct = syn::parse(input.into()).map_err(|_| "Only named structs objects can be extended".to_string())?;
        let target_struct_ident = &target_struct.ident.clone();
        let mut type_map = HashMap::<Type, Ident>::new(); // associate extending type to struct field
        
        // TODO handle unnamed fields correctly
        //let mut fields = Vec::<Field>::with_capacity(target_struct.fields.len());
        for field in target_struct.fields.iter_mut() {
            // parent attributes must be removed after processing
            // this stores any remaining attributes (which may be used by other macros or the compiler)
            let mut new_attributes = Vec::with_capacity(field.attrs.len());
            
            // associate field type with field ident if has #[roopert(parent)] or #[parent] attr
            for attr in &field.attrs {
                let is_parent_path = is_parent_attribute(attr);
                let is_roopert_path = is_roopert_attribute(attr);
                let mut parent_attr: Option<ParentAttribute> = None;
                if is_parent_path {
                    parent_attr = Some(attr.parse_args::<ParentAttribute>().map_err(|_| "Malformed roopert #[parent] attribute".to_string())?);
                } else if is_roopert_path {
                    let parsed_attr = attr.parse_args::<RoopertAttribute>().map_err(|_| "Malformed #[roopert(parent)] attribute".to_string())?;
                    if !parsed_attr.attr.is_parent() { 
                        new_attributes.push(attr.clone()); // not parent attribute, keep it
                        continue;
                    }
                    parent_attr = Some(match parsed_attr.attr {
                        RoopertAttributeType::Parent(a) => Ok(a),
                        _ => Err("Encountered quantum superpositioned #[roopert(???)] attribute (is_extends() -> true but not Extends)".to_string())
                    }?);
                } else {
                    new_attributes.push(attr.clone()); // not roopert-related attribute, keep it
                }
                if parent_attr.is_some() {
                    type_map.insert(field.ty.clone(), field.ident.clone().unwrap());
                    break;
                }
            }
            field.attrs = new_attributes;
            if !type_map.contains_key(&field.ty) {
                type_map.insert(field.ty.clone(), field.ident.clone().unwrap());
            }
        }
        
        // generate new code
        let mut tokens = vec![quote!{#target_struct}];
        for parent_type in self.types.iter() {
            let target_field = match type_map.get(parent_type) {
                Some(x) => Ok(x),
                None => Err("Cannot extend type not which is not also a field of this struct".to_string())
            }?;
            
            // AsRef implementation
            let token = Self::impl_asref(target_struct_ident, target_field, parent_type);
            tokens.push(token);
            
            // AsMut implementation
            let token = Self::impl_asmut(target_struct_ident, target_field, parent_type);
            tokens.push(token);
            
            // Into implementation
            let token = Self::impl_into(target_struct_ident, target_field, parent_type);
            tokens.push(token);
            
            // Deref implementation
            let token = Self::impl_deref(target_struct_ident, target_field, parent_type);
            tokens.push(token);
            
            // DerefMut implementation
            let token = Self::impl_derefmut(target_struct_ident, target_field, parent_type);
            tokens.push(token);
        }
        Ok(quote!{
            #(#tokens)*
        })
    }
    
    fn auto_append(&self) -> bool {false}
}
