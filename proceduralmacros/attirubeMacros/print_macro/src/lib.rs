use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, ItemFn };

#[proc_macro_attribute]
pub fn print_input(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    let generate =
    // reconstructing the original function declaration
        quote! {
        #vis #sig {
            println!("calling function : {}", stringify!(#fn_name));
            #block
        }
    };
    generate.into()
}


// working: -
/*
    takes a function
    wraps its body to print the function name
    re-inserts the original body after the print
*/