use crate::build_error;
use proc_macro2::TokenStream;
use syn::{FieldsNamed, GenericArgument, PathArguments, Type};

const RENDERABLES_IDENT: &str = "renderables";
const RENDERABLE_TYPE: &str = "Renderable";

pub fn validate(fields: &FieldsNamed) -> Result<(), TokenStream> {
    for field in &fields.named {
        let ident = field
            .ident
            .as_ref()
            .expect("named fields always have identifiers");

        if ident == RENDERABLES_IDENT {
            let error_msg = &format!(
                "the type of the \"{}\" field must be \"Vec<{}>\"",
                RENDERABLES_IDENT, RENDERABLE_TYPE
            );

            match &field.ty {
                Type::Path(path) => {
                    let path = &path.path;

                    let segment = match path.segments.first() {
                        Some(segment) => segment,
                        None => return Err(build_error(path, error_msg)),
                    };

                    if &segment.ident != "Vec" {
                        return Err(build_error(segment, error_msg));
                    }

                    let arguments = match &segment.arguments {
                        PathArguments::AngleBracketed(arguments) => &arguments.args,
                        _ => return Err(build_error(&segment.arguments, error_msg)),
                    };

                    let arg = match arguments.first() {
                        Some(arg) => arg,
                        None => return Err(build_error(arguments, error_msg)),
                    };

                    let ty = match arg {
                        GenericArgument::Type(ty) => ty,
                        _ => return Err(build_error(arg, error_msg)),
                    };

                    let path = match ty {
                        Type::Path(path) => &path.path,
                        _ => return Err(build_error(ty, error_msg)),
                    };

                    let segment = match path.segments.first() {
                        Some(segment) => segment,
                        None => return Err(build_error(path, error_msg)),
                    };

                    if segment.ident != RENDERABLE_TYPE {
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
            "a component should have a \"{}: Vec<{}>\" field",
            RENDERABLES_IDENT, RENDERABLE_TYPE
        ),
    ))
}
