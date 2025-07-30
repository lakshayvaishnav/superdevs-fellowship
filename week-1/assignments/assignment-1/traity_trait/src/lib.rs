use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Attribute,
    Data,
    DataStruct,
    DeriveInput,
    Error,
    Fields,
    FieldsNamed,
};



fn todo_app_impl(mut input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    //1. make sure it's a struct with named fields
    let ds = match &input.data {
        //“If input.data is a Data::Struct, and that struct has named fields,
        // then bind the full DataStruct to a variable called ds, and return ds.”
        // .. means ignore the rest of the fields in DataStruct.
        // (_) means ignores the actual named fields -- we only care that they're named.
        Data::Struct(ds @ DataStruct { fields: Fields::Named(_), .. }) => ds,
        _ => {
            return Err(
                Error::new_spanned(
                    input.ident,
                    "#[todo_app] can only be applied to structs with named fields"
                )
            );
        }
    };

    // 2. remove our own attribute so we don't re-emit it
    input.attrs.retain(|attr: &Attribute| {
        // keep all attrs except the one whose path is "todo_app"
        !attr.path().is_ident("todo_app")
    });

    // 3. Build the final struct + attributes
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = &ds.fields;
    let where_clause = &generics.where_clause;

    Ok(
        quote! {
        #(#attrs)*
        #[serde(rename_all = "PascalCase")]
        #vis struct #ident #generics
            #fields
        #where_clause
    }
    )
}

#[proc_macro_attribute]
pub fn todo_app(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match todo_app_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
