mod child_component;
mod field_validator;

use child_component::Attr;
use field_validator::{renderable, transform};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::result;
use syn::{
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(Component, attributes(child, children))]
pub fn component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match gen_component_getter_impl(&input) {
        Ok(getter_impl) => getter_impl,
        Err(compile_error) => compile_error,
    }
    .into()
}

fn gen_component_getter_impl(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let data_struct = extract_data_struct(input)?;
    let fields = extract_named_fields(data_struct)?;

    transform::validate(fields)?;
    renderable::validate(fields)?;

    let child_cmp_fields = child_component::parse_fields(fields)?;
    let children_impl = gen_children_impl(&child_cmp_fields);
    let children_mut_impl = gen_children_mut_impl(&child_cmp_fields);

    let ident = &input.ident;

    Ok(quote! {
        impl twors::ComponentGetter for #ident {
            fn transform(&self) -> &twors::Transform {
                &self.transform
            }

            fn transform_mut(&mut self) -> &mut twors::Transform {
                &mut self.transform
            }

            fn renderables(&self) -> &[twors::Renderable] {
                &self.renderables
            }

            fn children(&self) -> Vec<&dyn twors::Component> {
                #children_impl
            }

            fn children_mut(&mut self) -> Vec<&mut dyn twors::Component> {
                #children_mut_impl
            }
        }

    })
}

fn extract_data_struct(input: &DeriveInput) -> result::Result<&DataStruct, TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => Ok(data_struct),
        _ => Err(build_error(
            &input,
            "the macro should only be used on structs",
        )),
    }
}

fn extract_named_fields(data_struct: &DataStruct) -> result::Result<&FieldsNamed, TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields) => Ok(fields),
        _ => Err(build_error(
            &data_struct.fields,
            "the struct should contain only named fields",
        )),
    }
}

enum Mutability {
    Immutable,
    Mutable,
}

fn gen_children_impl(fields: &[child_component::Field]) -> TokenStream {
    gen_children_parameterized(fields, Mutability::Immutable)
}

fn gen_children_mut_impl(fields: &[child_component::Field]) -> TokenStream {
    gen_children_parameterized(fields, Mutability::Mutable)
}

fn gen_children_parameterized(
    fields: &[child_component::Field],
    mutability: Mutability,
) -> TokenStream {
    if fields.is_empty() {
        return quote!(Vec::default());
    }

    let ref_type = match mutability {
        Mutability::Immutable => quote!(&),
        Mutability::Mutable => quote!(&mut),
    };

    let iter_type = match mutability {
        Mutability::Immutable => quote!(.iter()),
        Mutability::Mutable => quote!(.iter_mut()),
    };

    let mut extensions = Vec::default();
    for field in fields {
        match field.attr {
            Attr::Child => {
                let ident = &field.field.ident;
                extensions.push(quote! {
                    children.push(
                        #ref_type self.#ident as #ref_type dyn twors::Component
                    );
                });
            }
            Attr::Children => {
                let ident = &field.field.ident;
                extensions.push(quote! {
                    children.extend(
                        self.#ident
                            #iter_type
                            .map(|cmp| cmp as #ref_type dyn twors::Component),
                    );
                })
            }
        }
    }

    quote! {
        let mut children = Vec::default();
        #(#extensions)*
        children
    }
}

fn build_error(item: &dyn Spanned, error: &str) -> TokenStream {
    let error = format!("\"component\" macro error: {}.", error);
    quote_spanned! {
        item.span() => compile_error!(#error);
    }
}
