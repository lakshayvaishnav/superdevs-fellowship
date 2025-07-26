use proc_macro::TokenStream;
use quote::quote;
use syn::{ self, DeriveInput };

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // parse the input as token stream;
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generate =
        quote! {
        impl HelloMacro for #name {
            fn hello()  {
                println!("hii your name is : {}", stringify!(#name));
            }
        }
    };

    generate.into()
}
