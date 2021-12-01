use syn::{Result, Ident, Type, Expr, punctuated::Punctuated, Token, Path, Lit};
use syn::parse::{Parse, ParseStream};

use quote::{quote, format_ident, ToTokens};

use proc_macro2::{TokenStream};

use super::Generate;
use super::parse::single_path_segment;

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct GetterAttribute {
    // TODO
    pre: Option<Expr>,
    mutable: bool,
}

impl GetterAttribute {
    pub fn with_accessor_defaults() -> Self {
        Self {
            pre: None,
            mutable: false,
        }
    }
    
    pub fn impl_get_fn(&self, target_field: &Ident, parent_type: &Type) -> TokenStream {
        let getter_fn_name = format_ident!("get_{}", target_field);
        let pre_op = match self.pre.as_ref() {
            Some(op) => quote!{#op;}.to_token_stream(),
            None => quote!{}.to_token_stream()
        };
        if self.mutable {
            quote!{
                pub fn #getter_fn_name(&mut self) -> &'_ mut #parent_type {
                    #pre_op
                    &mut self.#target_field
                }
            }
        } else {
            quote!{
                pub fn #getter_fn_name(&self) -> &'_ #parent_type {
                    #pre_op
                    &self.#target_field
                }
            }
        }
    }
    
    #[inline]
    fn rhs_to_bool(rhs: &Expr, input: ParseStream) -> Result<bool> {
        match rhs {
            Expr::Lit(lit) => {
                match &lit.lit {
                    Lit::Bool(lit_bool) => {
                        Ok(lit_bool.value)
                    },
                    Lit::Str(lit_str) => {
                        match &lit_str.value().to_lowercase() as &str {
                            "true" => Ok(true),
                            "false" => Ok(false),
                            _ => Err(input.error(format!("Invalid string literal in assignment #[roopert(get, ... = {})]", lit.to_token_stream())))
                        }
                    },
                    _ => Err(input.error(format!("Unrecognised literal type in right hand side of assignment #[roopert(get, ... = {})] (use \"true\", true, \"false\", or false)", rhs.to_token_stream())))
                }
            },
            _ => Err(input.error(format!("Unrecognised right hand side of assignment {} #[roopert(get, ... = {})] (use \"true\", true, \"false\", or false)", rhs.to_token_stream(), rhs.to_token_stream())))
        }
    }
}

impl Parse for GetterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut mutable = false;
        let mut pre_effect = None;
        println!("Input: {}", input);
        let params = Punctuated::<Expr, Token![,]>::parse_terminated(input).map_err(|e| input.error(format!("Invalid parameter in #[roopert(get, ...)]: {}", e)))?;
        for param in params.iter() {
            match param {
                Expr::Assign(assign) => {
                    if let Expr::Path(var) = &*assign.left {
                        let ident = single_path_segment(&var.path, input, getter_lhs_err)?;
                        match &ident.to_string().to_lowercase() as &str {
                            "pre" => {
                                pre_effect = Some((&*assign.right).clone());
                                Ok(())
                            },
                            "mut" | "mut_" | "mutable" => {
                                mutable = Self::rhs_to_bool(&*assign.right, input)?;
                                Ok(())
                            },
                            _ => Err(input.error(format!("Unrecognised left hand side of assignment {} in #[roopert(get, ...)]", ident.to_string())))
                        }
                    } else {
                        Err(input.error(format!("Unsupported left hand side of assignment {} in #[roopert(get, ...)]", assign.to_token_stream())))
                    }
                },
                _ => Err(input.error(format!("Unrecognised attribute parameter {} in #[roopert(get, ...)]", param.to_token_stream())))
            }?;
        }
        Ok(Self{
            pre: pre_effect,
            mutable: mutable,
        })
    }
}

impl Generate for GetterAttribute {
    fn generate(&mut self, _input: TokenStream) -> core::result::Result<TokenStream, String> {
        Ok(quote!{}.into())
    }
    
    fn auto_append(&self) -> bool {true}
}

fn getter_lhs_err(path: &Path) -> String {
    format!("Unrecognised left hand side of assignment {} in #[roopert(get, ...)]", path.to_token_stream())
}
