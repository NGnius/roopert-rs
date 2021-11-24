use proc_macro2::TokenStream;
use syn::{Ident, Result, Token};
use syn::parse::{Parse, ParseStream};

use super::{ParentAttribute, ExtendsAttribute, AccessorsAttribute, GetterAttribute, SetterAttribute, Generate};

#[cfg_attr(feature="verbose", derive(Debug))]
pub enum RoopertAttributeType {
    Parent(ParentAttribute),
    Extends(ExtendsAttribute),
    Accessors(AccessorsAttribute),
    Get(GetterAttribute),
    Set(SetterAttribute),
}

impl RoopertAttributeType {
    pub fn is_parent(&self) -> bool {
        match self {
            Self::Parent(_) => true,
            _ => false,
        }
    }
    
    pub fn is_extends(&self) -> bool {
        match self {
            Self::Extends(_) => true,
            _ => false,
        }
    }
    
    pub fn is_getter(&self) -> bool {
        match self {
            Self::Get(_) => true,
            _ => false
        }
    }
    
    pub fn is_setter(&self) -> bool {
        match self {
            Self::Set(_) => true,
            _ => false
        }
    }
}

impl Generate for RoopertAttributeType {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        match self {
            Self::Parent(parent) => parent.generate(input),
            Self::Extends(extends) => extends.generate(input),
            Self::Accessors(accessors) => accessors.generate(input),
            Self::Get(getters) => getters.generate(input),
            Self::Set(setters) => setters.generate(input),
        }
    }
    
    fn auto_append(&self) -> bool {
        match self {
            Self::Parent(parent) => parent.auto_append(),
            Self::Extends(extends) => extends.auto_append(),
            Self::Accessors(accessors) => accessors.auto_append(),
            Self::Get(getters) => getters.auto_append(),
            Self::Set(setters) => setters.auto_append(),
        }
    }
}

#[cfg_attr(feature="verbose", derive(Debug))]
pub struct RoopertAttribute {
    //ident: Ident,
    pub attr: RoopertAttributeType,
}

impl Parse for RoopertAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        // ignore comma -- just a separator
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
        }
        match &ident.to_string() as &str {
            "parent" => Ok(Self {
                    //ident: ident,
                    attr: RoopertAttributeType::Parent(ParentAttribute::parse(input)?),
                }),
            "extend" | "extends" => Ok(Self {
                    //ident: ident,
                    attr: RoopertAttributeType::Extends(ExtendsAttribute::parse(input)?),
                }),
            "accessors" => Ok(Self {
                //ident: ident,
                attr: RoopertAttributeType::Accessors(AccessorsAttribute::parse(input)?),
            }),
            "get" => Ok(Self {
                //ident: ident,
                attr: RoopertAttributeType::Get(GetterAttribute::parse(input)?),
            }),
            "set" => Ok(Self {
                //ident: ident,
                attr: RoopertAttributeType::Set(SetterAttribute::parse(input)?),
            }),
            _ => Err(input.error(format!("unexpected identifier {}", ident.to_string())))
        }
    }
}

impl Generate for RoopertAttribute {
    fn generate(&mut self, input: TokenStream) -> core::result::Result<TokenStream, String> {
        self.attr.generate(input)
    }
    
    fn auto_append(&self) -> bool {self.attr.auto_append()}
}
