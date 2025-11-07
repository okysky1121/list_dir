use std::{fs, path::Path};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Ident, LitStr, Result, Token,
};

struct ListDir(Ident, LitStr);

impl Parse for ListDir {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let path: LitStr = input.parse()?;
        if path.value().is_empty() {
            return Err(Error::new(path.span(), "`path` must not be empty"));
        }

        Ok(ListDir(name, path))
    }
}

#[proc_macro]
pub fn list_dir(input: TokenStream) -> TokenStream {
    let ListDir(name, path) = parse_macro_input!(input);

    let path = path.value();
    let path = Path::new(&path);
    let list = fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.file_name().into_string().unwrap())
        .collect::<Vec<String>>();
    let size = list.len();

    quote! {
        const #name: [&str; #size] = [#(#list),*];
    }
    .into()
}
