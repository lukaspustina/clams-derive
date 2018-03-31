#![recursion_limit="128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Config)]
pub fn config(input: TokenStream) -> TokenStream {
     let input: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_config(&input);

    // Return the generated impl
    gen.into()
}

fn impl_config(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;
        use toml;
        impl Config for #name {
            type ConfigStruct = #name;

            fn from_file<T: AsRef<Path>>(file_path: T) -> Result<Self::ConfigStruct> {
                let mut file = File::open(file_path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let config: Self::ConfigStruct = toml::from_str(&content)?;

                Ok(config)
            }
        }
    }
}

