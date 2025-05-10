use crate::build_error;
use proc_macro2::TokenStream;
use syn::{FieldsNamed, Type};

const TRANSFORM_IDENT: &str = "transform";
const TRANSFORM_TYPE: &str = "Transform";

pub fn validate(fields: &FieldsNamed) -> Result<(), TokenStream> {
    for field in &fields.named {
        let ident = field
            .ident
            .as_ref()
            .expect("named fields always have identifiers");

        if ident == TRANSFORM_IDENT {
            let error_msg = &format!(
                "the type of the \"{}\" field must be \"{}\"",
                TRANSFORM_IDENT, TRANSFORM_TYPE
            );

            match &field.ty {
                Type::Path(path) => {
                    let path = &path.path;

                    let segment = match path.segments.first() {
                        Some(segment) => segment,
                        None => return Err(build_error(path, error_msg)),
                    };

                    if segment.ident != TRANSFORM_TYPE {
                        return Err(build_error(segment, error_msg));
                    }
                }
                _ => {
                    return Err(build_error(field, error_msg));
                }
            }

            return Ok(());
        }
    }

    Err(build_error(
        &fields,
        &format!(
            "a component should have a \"{}: {}\" field",
            TRANSFORM_IDENT, TRANSFORM_TYPE
        ),
    ))
}
