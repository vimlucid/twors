use crate::build_error;
use proc_macro2::TokenStream;
use syn::{FieldsNamed, Meta};

pub const CHILD_ATTR: &str = "child";
pub const CHILDREN_ATTR: &str = "children";

pub struct Field {
    pub field: syn::Field,
    pub attr: Attr,
}

pub enum Attr {
    Child,
    Children,
}

impl TryFrom<&str> for Attr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            CHILD_ATTR => Ok(Attr::Child),
            CHILDREN_ATTR => Ok(Attr::Children),
            other => Err(format!("Invalid ChildComponentAttr value \"{}\"", other)),
        }
    }
}

// TODO: Validate Vec on components
pub fn parse_fields(fields: &FieldsNamed) -> Result<Vec<Field>, TokenStream> {
    let mut child_cmp_fields = Vec::default();

    for field in &fields.named {
        if field.attrs.is_empty() {
            continue;
        }

        if field.attrs.len() > 1 {
            return Err(build_error(field, "expected a single attribute per field"));
        }

        let unsupported_attr_err_msg =
            format!("expected \"#[{}]\" or \"#[{}]\"", CHILD_ATTR, CHILDREN_ATTR);

        let attr = field.attrs.first().unwrap();
        let path = match &attr.meta {
            Meta::Path(path) => path,
            _ => return Err(build_error(attr, &unsupported_attr_err_msg)),
        };

        if path.segments.len() != 1 {
            return Err(build_error(path, &unsupported_attr_err_msg));
        }
        let segment = path
            .segments
            .first()
            .expect("we just checked that there's exacltly one segment");

        let attr = match Attr::try_from(segment.ident.to_string().as_ref()) {
            Ok(attr) => attr,
            Err(_) => return Err(build_error(&segment.ident, &unsupported_attr_err_msg)),
        };

        child_cmp_fields.push(Field {
            field: field.clone(),
            attr,
        });
    }

    Ok(child_cmp_fields)
}
