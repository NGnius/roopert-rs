use syn::{Result, Ident, Type, Expr, Token, punctuated::Punctuated, Path};
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
}

impl SetterAttribute {
    pub fn with_accessor_defaults() -> Self {
        Self {
            pre: None,
            post: None,
        }
    }
    
    pub fn impl_set_fn(&self, target_field: &Ident, parent_type: &Type) -> TokenStream {
        let setter_fn_name = format_ident!("set_{}", target_field);
        let pre_op = match self.pre.as_ref() {
            Some(op) => op.to_token_stream(),
            None => quote!{}.to_token_stream()
        };
        let post_op = match self.post.as_ref() {
            Some(op) => op.to_token_stream(),
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
}

impl Parse for SetterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut pre_effect = None;
        let mut post_effect = None;
        let params = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
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
                            _ => Err(input.error(format!("Unrecognised left hand side of assignment {} in #[roopert(set, ...)]", ident.to_string())))
                        }
                    } else {
                        Err(input.error(format!("Unsupported left hand side of assignment {} in #[roopert(set, ...)]", assign.to_token_stream())))
                    }
                },
                _ => Err(input.error(format!("Unrecognised attribute parameter {} in #[roopert(set, ...)]", param.to_token_stream())))
            }?;
        }
        Ok(Self{
            pre: pre_effect,
            post: post_effect,
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
