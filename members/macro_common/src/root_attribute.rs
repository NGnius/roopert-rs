use proc_macro2::TokenStream;
use syn::{Ident, Result, Token};
use syn::parse::{Parse, ParseStream};

use super::{ParentAttribute, ExtendsAttribute, Generate};

#[cfg_attr(feature="verbose", derive(Debug))]
pub enum RoopertAttributeType {
    Parent(ParentAttribute),
    Extends(ExtendsAttribute),
}

impl RoopertAttributeType {
    pub fn is_parent(&self) -> bool {
        match self {
            Self::Parent(_) => true,
            Self::Extends(_) => false,
        }
    }
    
    pub fn is_extends(&self) -> bool {
        match self {
            Self::Parent(_) => false,
            Self::Extends(_) => true,
        }
    }
}

impl Generate for RoopertAttributeType {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        match self {
            Self::Parent(parent) => parent.generate(input),
            Self::Extends(extends) => extends.generate(input),
        }
    }
    
    fn auto_append(&self) -> bool {true}
}

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct RoopertAttribute {
    ident: Ident,
    pub attr: RoopertAttributeType,
}

impl Parse for RoopertAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token![,] = input.parse()?;
        match &ident.to_string() as &str {
            "parent" => Ok(Self {
                    ident: ident,
                    attr: RoopertAttributeType::Parent(ParentAttribute::parse(input)?),
                }),
            "extend" | "extends" => Ok(Self {
                    ident: ident,
                    attr: RoopertAttributeType::Extends(ExtendsAttribute::parse(input)?)
                }),
            _ => Err(input.error(format!("unexpected identifier {}", ident.to_string())))
        }
    }
}

impl Generate for RoopertAttribute {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        self.attr.generate(input)
    }
    
    fn auto_append(&self) -> bool {true}
}
