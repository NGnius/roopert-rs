use syn::{Result, Ident, Type, Expr, Token, punctuated::Punctuated, Path, Lit};
use syn::parse::{Parse, ParseStream};

use quote::{quote, format_ident, ToTokens};

use proc_macro2::{TokenStream};

use super::Generate;
use super::parse::single_path_segment;

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct SetterAttribute {
    // TODO
    pre: Option<Expr>,
    post: Option<Expr>,
    name: Option<String>,
}

impl SetterAttribute {
    pub fn with_accessor_defaults() -> Self {
        Self {
            pre: None,
            post: None,
            name: None,
        }
    }
    
    pub fn impl_set_fn(&self, target_field: &Ident, parent_type: &Type) -> TokenStream {
        let setter_fn_name = self.name.as_ref()
            .and_then(|name| Some(format_ident!("{}", name)))
            .unwrap_or_else(|| format_ident!("set_{}", target_field));
        let pre_op = match self.pre.as_ref() {
            Some(op) => quote!{#op;}.to_token_stream(),
            None => quote!{}.to_token_stream()
        };
        let post_op = match self.post.as_ref() {
            Some(op) => quote!{#op;}.to_token_stream(),
            None => quote!{}.to_token_stream()
        };
        quote!{
            pub fn #setter_fn_name(&mut self, x: #parent_type) {
                #pre_op
                self.#target_field = x;
                #post_op
            }
        }
    }
    
    #[inline]
    fn name_to_string(rhs: &Expr, input: ParseStream) -> Result<String> {
        match rhs {
            Expr::Lit(lit) => {
                match &lit.lit {
                    Lit::Str(lit_str) => Ok(lit_str.value()),
                    _ => Err(input.error(format!("Invalid literal in right hand side of name parameter #[roopert(set, name = {})]", lit.to_token_stream())))
                }
            },
            _ => Err(input.error(format!("Unrecognised right hand side of name parameter #[roopert(set, name = {})]", rhs.to_token_stream())))
        }
    }
}

impl Parse for SetterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut pre_effect = None;
        let mut post_effect = None;
        let mut name = None;
        let params = Punctuated::<Expr, Token![,]>::parse_terminated(input).map_err(|e| input.error(format!("Invalid parameter in #[roopert(set, ...)]: {}", e)))?;
        for param in params.iter() {
            match param {
                Expr::Assign(assign) => {
                    if let Expr::Path(var) = &*assign.left {
                        let ident = single_path_segment(&var.path, input, setter_lhs_err)?;
                        match &ident.to_string().to_lowercase() as &str {
                            "pre" => {
                                pre_effect = Some((&*assign.right).clone());
                                Ok(())
                            },
                            "post" => {
                                post_effect = Some((&*assign.right).clone());
                                Ok(())
                            },
                            "name" => {
                                name = Some(Self::name_to_string(&*assign.right, input)?);
                                Ok(())
                            }
                            _ => Err(input.error(format!("Unrecognised left hand side of assignment {} in #[roopert(set, ...)]", ident.to_string())))
                        }
                    } else {
                        Err(input.error(format!("Unsupported left hand side of assignment {} in #[roopert(set, ...)]", assign.to_token_stream())))
                    }
                },
                _ => Err(input.error(format!("Unrecognised attribute parameter {} in #[roopert(set, ...)]", param.to_token_stream())))
            }?;
        }
        Ok(Self {
            pre: pre_effect,
            post: post_effect,
            name: name,
        })
    }
}

impl Generate for SetterAttribute {
    fn generate(&mut self, _input: TokenStream) -> core::result::Result<TokenStream, String> {
        Ok(quote!{}.into())
    }
    
    fn auto_append(&self) -> bool {true}
}

fn setter_lhs_err(path: &Path) -> String {
    format!("{}", path.to_token_stream())
}
