use proc_macro2::{TokenStream};

use syn::{ItemStruct, Field, Ident, Result, Token, punctuated::Punctuated, Type, Visibility, Expr, Path, Lit};
use syn::parse::{Parse, ParseStream};

use quote::{quote, ToTokens};

use super::{Generate, RoopertAttribute, RoopertAttributeType, GetterAttribute, SetterAttribute};

use super::parse::{is_getter_attribute, is_setter_attribute, is_roopert_attribute, single_path_segment};

#[cfg_attr(feature="verbose", derive(Debug))]
enum AccessorAutoRule {
    All,
    Private,
    No,
}

#[derive(Clone)]
struct FieldMetadata {
    ty: Type,
    ident: Ident,
}

impl FieldMetadata {
    fn from_named_field(field: &Field) -> Self {
        Self {
            ty: field.ty.clone(),
            ident: field.ident.clone().unwrap(),
        }
    }
}

impl AccessorAutoRule {
    fn needs_accessor(&self, field: &Field) -> bool {
        match self {
            AccessorAutoRule::All => true,
            AccessorAutoRule::Private => field.vis == Visibility::Inherited,
            AccessorAutoRule::No => false,
        }
    }
    
    fn from_assignment_str(value: &str, input: ParseStream, ctx: &str) -> Result<AccessorAutoRule> {
        match value {
            "all" => Ok(AccessorAutoRule::All),
            "private" => Ok(AccessorAutoRule::Private),
            "no" => Ok(AccessorAutoRule::No),
            _ => Err(input.error(format!("Unrecognised right hand side of assignment in #[roopert(accesssor, ..., {} = {}]", ctx, value)))
        }
    }
}

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct AccessorsAttribute {
    getter_rule: AccessorAutoRule,
    setter_rule: AccessorAutoRule,
}

impl AccessorsAttribute {
    fn rule_from_expr(assignee: &Ident, expr: &Expr, input: ParseStream) -> Result<AccessorAutoRule> {
        match expr {
            Expr::Path(var) => 
                AccessorAutoRule::from_assignment_str(&single_path_segment(&var.path, input, accessor_path_err_rule)?.to_string().to_lowercase(), input, &assignee.to_string()),
            Expr::Lit(literal) => {
                match &literal.lit {
                    Lit::Str(lit_str) => 
                        AccessorAutoRule::from_assignment_str(&lit_str.value().to_lowercase(), input, &assignee.to_string()),
                    //Lit::Int(lit_int) => {},
                    _ => Err(input.error(format!("Unsupported literal type in right hand side of assignment in #[roopert(accessor, ..., {} = ???]", assignee.to_string())))
                }
            },
            _ => Err(input.error(format!("Unrecognised right hand side of assignment in #[roopert(accessor, ..., {} = ???)]", assignee.to_string())))
        }
    }
}

impl Parse for AccessorsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse for optional get and set rules
        let mut get_rule = None;
        let mut set_rule = None;
        let params = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        for p in params.iter() {
            match p {
                Expr::Assign(assign) => {
                    if let Expr::Path(var) = &*assign.left {
                        let ident = single_path_segment(&var.path, input, accessor_path_err_left)?;
                        match &ident.to_string().to_lowercase() as &str {
                            "get" => {
                                get_rule = Some(Self::rule_from_expr(&ident, &*assign.right, input)?);
                                Ok(())
                            },
                            "set" => {
                                set_rule = Some(Self::rule_from_expr(&ident, &*assign.right, input)?);
                                Ok(())
                            }
                            _ => Err(input.error(format!("Unrecognised assignment {} in #[roopert(accessor, ...)]", ident.to_string())))
                        }
                    } else {
                        Err(input.error("Unsupported left hand side of assignment in #[roopert(accessor, ..., ??? = ...]"))
                    }
                },
                _ => Err(input.error(format!("Unrecognised attribute parameter {} in #[roopert(accessor, ...)]", p.to_token_stream())))
            }?;
        }
        Ok(Self{
            getter_rule: get_rule.unwrap_or(AccessorAutoRule::No),
            setter_rule: set_rule.unwrap_or(AccessorAutoRule::No),
        })
    }
}

impl Generate for AccessorsAttribute {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        //self.attr.generate(input)
        let mut target_struct: ItemStruct = syn::parse(input.into()).map_err(|_| "Only named structs objects can have roopert accessors".to_string())?;
        let target_struct_ident = &target_struct.ident.clone();
        let mut getters: Vec<(FieldMetadata, GetterAttribute)> = Vec::new();
        let mut setters: Vec<(FieldMetadata, SetterAttribute)> = Vec::new();
        
        // find getter and setter attributes
        for field in target_struct.fields.iter_mut() {
            let mut setter_found = false;
            let mut getter_found = false;
            let field_meta = FieldMetadata::from_named_field(field);
            
            // get and set attributes must be removed after processing
            // this stores any remaining attributes (which may be used by other macros or the compiler)
            let mut new_attributes = Vec::with_capacity(field.attrs.len());
            
            // associate field type with field ident if has #[roopert(parent)] or #[parent] attr
            for attr in &field.attrs {
                let is_getter_path = is_getter_attribute(attr);
                let is_setter_path = is_setter_attribute(attr);
                let is_roopert_path = is_roopert_attribute(attr);
                if is_getter_path {
                    let getter = attr.parse_args::<GetterAttribute>().map_err(|_| "Malformed roopert #[get] attribute".to_string())?;
                    getter_found = true;
                    getters.push((field_meta.clone(), getter));
                } else if is_setter_path {
                    let setter = attr.parse_args::<SetterAttribute>().map_err(|_| "Malformed roopert #[set] attribute".to_string())?;
                    setter_found = true;
                    setters.push((field_meta.clone(), setter));
                } else if is_roopert_path {
                    let parsed_attr = attr.parse_args::<RoopertAttribute>().map_err(|e| format!("Malformed #[roopert(...)] attribute: {}", e))?;
                    match parsed_attr.attr {
                        RoopertAttributeType::Get(getter) => {
                            getter_found = true;
                            getters.push((
                                field_meta.clone(), getter
                            ))
                        },
                        RoopertAttributeType::Set(setter) => {
                            setter_found = true;
                            setters.push((
                                field_meta.clone(), setter
                            ))
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
            if !setter_found && self.setter_rule.needs_accessor(field) {
                setters.push((field_meta.clone(), SetterAttribute::with_accessor_defaults()));
            }
            
            if !getter_found && self.getter_rule.needs_accessor(field) {
                getters.push((field_meta.clone(), GetterAttribute::with_accessor_defaults()));
            }
        }
        
        // generate accessors
        let mut getter_tokens = Vec::new();
        for (meta, attr) in getters {
            getter_tokens.push(attr.impl_get_fn(&meta.ident, &meta.ty));
        }
        let mut setter_tokens = Vec::new();
        for (meta, attr) in setters {
            setter_tokens.push(attr.impl_set_fn(&meta.ident, &meta.ty));
        }
        let (impl_generics, ty_generics, where_clause) = target_struct.generics.split_for_impl();
        Ok(quote!{
            #target_struct
            
            impl #impl_generics #target_struct_ident #ty_generics #where_clause {
                #(#getter_tokens)*
                
                #(#setter_tokens)*
            }
        })
    }
    
    fn auto_append(&self) -> bool {false}
}

fn accessor_path_err_left(path: &Path) -> String {
    format!("Unsupported path in left hand side of assignment in attribute #[roopert(accessors, ... = {})]", path.to_token_stream())
}


fn accessor_path_err_rule(path: &Path) -> String {
    format!("Unsupported path in right hand side of assignment in attribute #[roopert(accessors, ... = {})]", path.to_token_stream())
}
