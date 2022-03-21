use syn::Attribute;

pub fn is_parent_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "parent")
}

pub fn is_roopert_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "roopert")
}

pub fn is_getter_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "get")
}

pub fn is_setter_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "set")
}

pub fn is_metadata_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "metadata")
}

pub fn is_field_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "field")
}

pub fn is_fields_attribute(attr: &Attribute) -> bool {
    is_attribute(attr, "fields")
}

fn is_attribute(attr: &Attribute, last: &str) -> bool {
    match attr.path.segments.last() {
        Some(last_segment) => last_segment.ident.to_string() == last,
        None => false
    }
}
