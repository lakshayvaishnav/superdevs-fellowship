use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ Data, DeriveInput, Field, Fields };

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_serialize(&ast)
}

#[proc_macro_derive(DesrializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_deserialize(&ast)
}

fn impl_serialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let serialize_fields = match &ast.data {
        // makes sure it only handles struct.
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                // make sure the fields of struct are named.
                Fields::Named(fields) => {
                    let field_serializations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        quote! {
                                result.extend_from_slice(&self.#field_name.to_be_bytes());
                            }
                    });
                    // The repetition syntax #(...)* splices all of those per-field snippets in order.
                    quote! { #(#field_serializations)* }
                }
                _ => panic!("only named fields are supported"),
            }
        }
        _ => panic!("only structs are supported"),
    };

    let generated =
        quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };
    generated.into()
}

fn impl_deserialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let (deserialize_fields, field_assignments, total_size) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut offset: usize = 0;
                    let mut field_deserialization = Vec::new();
                    let mut field_assignments = Vec::new();

                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_size = 4;
                        let start_offest = offset;
                        let end_offset = offset + field_size;

                        field_deserialization.push(
                            quote! {
                            let #field.name = {
                                let bytes : [u8;4] = base[#start_offest..#end_offset].try_into().map_err(|_| Error)?;
                                i32::from_be_bytes(bytes)
                            };
                        }
                        );
                        field_assignments.push(
                            quote! {
                            #field_name
                        }
                        );

                        offset += field_size;
                    }
                    (field_deserialization, field_assignments, offset)
                }
                _ => panic!("only named fields are supported !"),
            }
        }
        _ => panic!("only structs are supported"),
    };

    let generated = quote! {
        impl Deserialize for #name {
            fn deserailize(base: &[u8]) -> Result<Self, Error> {
                if base.len() < #total_size {
                 return Err(Error)   
                }
                #(#deserialize_fields)*

                oK(#name {
                    #(#field_assignments,)*
                })
            }
        }
    };

    generated.into()
}
