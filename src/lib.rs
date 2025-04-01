use std::fs;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;

#[proc_macro]
#[proc_macro_error]
pub fn list_dir(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let path = &input.to_string();
    let iter = input.clone().into_iter().collect::<Vec<_>>();

    if iter.len() != 1 {
        abort!(input, "number of arguments is greater or less than 1")
    }

    match iter.last().unwrap() {
        TokenTree::Literal(_) => {
            let bytes = path.bytes().collect::<Vec<_>>();

            if bytes[0] != b'"' || *bytes.last().unwrap() != b'"' {
                abort!(input, "argument only accepts string(ex. \"/path/dir\")")
            }
        }
        _ => {
            abort!(input, "argument only accepts literal")
        }
    }

    let path = &path[1..(path.len() - 2)].to_string();

    if let Err(_) = fs::exists(path) {
        abort!(input, "maybe this path does not exists")
    }

    match fs::read_dir(path) {
        Ok(r) => {
            let list = r
                .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
                .collect::<Vec<_>>();

            return quote! { [#(#list),*] }.into();
        }
        Err(why) => abort!(input, "panicked while reading directory: {}", why),
    }
}
