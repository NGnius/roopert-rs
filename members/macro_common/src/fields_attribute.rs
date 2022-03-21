use syn::{ItemStruct, Field, Ident, Result, Token, punctuated::Punctuated, Visibility, Expr, Path, Lit};
use syn::parse::{Parse, ParseStream};

use quote::{quote, ToTokens};

use proc_macro2::{TokenStream};

use super::{Generate, RoopertAttribute, RoopertAttributeType};
use super::parse::{is_field_attribute, is_roopert_attribute, single_path_segment};

#[cfg_attr(feature="verbose", derive(Debug))]
#[derive(Clone)]
enum FieldsAutoRule {
    All,
    Public,
    No,
}

impl FieldsAutoRule {
    fn needs_accessor(&self, field: &Field) -> bool {
        match self {
            Self::All => true,
            Self::Public => if let Visibility::Public(_) = field.vis {true} else {false},
            Self::No => false,
        }
    }
    
    fn from_assignment_str(value: &str, input: ParseStream, ctx: &str) -> Result<Self> {
        match value {
            "all" => Ok(Self::All),
            "public" => Ok(Self::Public),
            "no" => Ok(Self::No),
            _ => Err(input.error(format!("Unrecognised right hand side of assignment in #[roopert(fields, ..., {} = {}]", ctx, value)))
        }
    }
}

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct FieldsAttribute {
    rule: FieldsAutoRule,
    metadata: Option<String>,
}

impl FieldsAttribute {
    fn rule_from_expr(assignee: &Ident, expr: &Expr, input: ParseStream) -> Result<FieldsAutoRule> {
        match expr {
            Expr::Path(var) => 
                FieldsAutoRule::from_assignment_str(&single_path_segment(&var.path, input, meta_path_err_rule)?.to_string().to_lowercase(), input, &assignee.to_string()),
            Expr::Lit(literal) => {
                match &literal.lit {
                    Lit::Str(lit_str) => 
                        FieldsAutoRule::from_assignment_str(&lit_str.value().to_lowercase(), input, &assignee.to_string()),
                    //Lit::Int(lit_int) => {},
                    _ => Err(input.error(format!("Unsupported literal type in right hand side of assignment in #[roopert(fields, ..., {} = ???]", assignee.to_string())))
                }
            },
            _ => Err(input.error(format!("Unrecognised right hand side of assignment in #[roopert(fields, ..., {} = ???)]", assignee.to_string())))
        }
    }
    
    fn metadata_from_expr(assignee: &Ident, expr: &Expr, input: ParseStream) -> Result<String> {
        match expr {
            Expr::Lit(literal) => {
                match &literal.lit {
                    Lit::Str(lit_str) => 
                        Ok(lit_str.value()),
                    //Lit::Int(lit_int) => {},
                    _ => Err(input.error(format!("Unsupported literal type in right hand side of assignment in #[roopert(fields, ..., {} = ???]", assignee.to_string())))
                }
            },
            _ => Err(input.error(format!("Unrecognised right hand side of assignment in #[roopert(fields, ..., {} = ???)]", assignee.to_string())))
        }
    }
}

impl Parse for FieldsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse for optional get and set rules
        let mut rule = None;
        let mut metadata_str = None;
        let params = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        for p in params.iter() {
            match p {
                Expr::Assign(assign) => {
                    if let Expr::Path(var) = &*assign.left {
                        let ident = single_path_segment(&var.path, input, meta_path_err_left)?;
                        match &ident.to_string().to_lowercase() as &str {
                            "rule" => {
                                rule = Some(Self::rule_from_expr(&ident, &*assign.right, input)?);
                                Ok(())
                            },
                            "metadata" | "meta" => {
                                metadata_str = Some(Self::metadata_from_expr(&ident, &*assign.right, input)?);
                                Ok(())
                            },
                            _ => Err(input.error(format!("Unrecognised assignment {} in #[roopert(fields, ...)]", ident.to_string())))
                        }
                    } else {
                        Err(input.error(format!("Unsupported left hand side of assignment in #[roopert(fields, ..., {}]", assign.to_token_stream())))
                    }
                },
                _ => Err(input.error(format!("Unrecognised attribute parameter {} in #[roopert(fields, ...)]", p.to_token_stream())))
            }?;
        }
        Ok(Self{
            rule: rule.unwrap_or(FieldsAutoRule::No),
            metadata: metadata_str,
        })
    }
}

struct FieldMetadata {
    meta: Option<String>,
    field: Field,
}

impl Generate for FieldsAttribute {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        let mut target_struct: ItemStruct = syn::parse(input.into()).map_err(|_| "Only named structs objects are supported for roopert fields".to_string())?;
        let target_struct_ident = &target_struct.ident.clone();
        
        let mut metadata = Vec::<FieldMetadata>::new();
        
        // find fields metadata
        for field in target_struct.fields.iter_mut() {
            let mut metadata_found = false;
            
            // metadata attributes must be removed after processing
            // this stores any remaining attributes (which may be used by other macros or the compiler)
            let mut new_attributes = Vec::with_capacity(field.attrs.len());
            
            // associate field type with field ident if has #[roopert(field)] or #[field] attr
            for attr in &field.attrs {
                let is_field_path = is_field_attribute(attr);
                let is_roopert_path = is_roopert_attribute(attr);
                if is_field_path {
                    let meta = attr.parse_args::<Self>().map_err(|_| "Malformed roopert #[field] attribute".to_string())?;
                    metadata_found = true;
                    metadata.push(FieldMetadata {
                        meta: meta.metadata,
                        field: field.clone(),
                    });
                } else if is_roopert_path {
                    let parsed_attr = attr.parse_args::<RoopertAttribute>().map_err(|e| format!("Malformed #[roopert(...)] attribute: {}", e))?;
                    match parsed_attr.attr {
                        RoopertAttributeType::FieldsMetadata(meta) => {
                            metadata_found = true;
                            metadata.push(FieldMetadata {
                                meta: meta.metadata,
                                field: field.clone(),
                            });
                        },
                        _ => {
                            new_attributes.push(attr.clone()); // keep non-related roopert attribute
                            continue;
                        }
                    }
                } else {
                    new_attributes.push(attr.clone()); // keep unrelated attribute
                }
            }
            field.attrs = new_attributes;
            if !metadata_found && self.rule.needs_accessor(field) {
                metadata.push(FieldMetadata {
                    meta: self.metadata.clone(),
                    field: field.clone(),
                });
            }
        }
        
        // generate Fields impl
        let mut match_arms_meta = Vec::with_capacity(metadata.len());
        let mut field_arr = Vec::with_capacity(metadata.len());
        let mut match_arms_get = Vec::with_capacity(metadata.len());
        let mut match_arms_get_mut = Vec::with_capacity(metadata.len());
        for meta in metadata {
            let field_name_str = meta.field.ident.as_ref().unwrap().to_string();
            let field_ident = meta.field.ident.as_ref().unwrap();
            // fn metadata()
            if let Some(m) = meta.meta {
                match_arms_meta.push(
                    quote!{#field_name_str => Some(#m)}
                );
            } else {
                match_arms_meta.push(
                    quote!{#field_name_str => None}
                );
            }
            // fn fields()
            field_arr.push(quote!{#field_name_str});
            // fn get()
            match_arms_get.push(quote!{#field_name_str => Some(&self.#field_ident)});
            match_arms_get_mut.push(quote!{#field_name_str => Some(&mut self.#field_ident)});
            // fn set()
            // TODO
        }
        Ok(quote!{
            #target_struct
            
            impl roopert::Fields for #target_struct_ident {
                fn fields(&self) -> &[&str] {
                    &[
                    #(#field_arr,)*
                    ]
                }
                
                fn get(&self, field_name: &str) -> Option<&dyn std::any::Any> {
                    match field_name {
                        #(#match_arms_get,)*
                        _ => None
                    }
                }
                
                fn get_mut(&mut self, field_name: &str) -> Option<&mut dyn std::any::Any> {
                    match field_name {
                        #(#match_arms_get_mut,)*
                        _ => None
                    }
                }
                
                fn metadata(&self, field_name: &str) -> Option<&str> {
                    match field_name {
                        #(#match_arms_meta,)*
                        _ => None
                    }
                }
            }
        })
    }
    
    fn auto_append(&self) -> bool {false}
}

fn meta_path_err_left(path: &Path) -> String {
    format!("Unsupported path in left hand side of assignment in attribute #[roopert(fields, ... = {})]", path.to_token_stream())
}


fn meta_path_err_rule(path: &Path) -> String {
    format!("Unsupported path in right hand side of assignment in attribute #[roopert(fields, ... = {})]", path.to_token_stream())
}
