#![recursion_limit = "128"]
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
        use std::path::Path;

        impl Config for #name {
            type ConfigStruct = #name;

            fn from_file<T: AsRef<Path>>(file_path: T) -> ConfigResult<Self::ConfigStruct> {
                use std::fs::File;
                use std::io::Read;
                use toml;

                let mut file = File::open(file_path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let config: Self::ConfigStruct = toml::from_str(&content)?;

                Ok(config)
            }

            fn smart_load<T: AsRef<Path>>(file_paths: &[T]) -> ConfigResult<Self::ConfigStruct> {
                for fp in file_paths {
                    let config = Self::from_file(fp);
                    if config.is_ok() { return config };
                }

                let failed_configs: Vec<String> = file_paths.iter().map(|x| x.as_ref().to_string_lossy().to_string()).collect();
                Err(ConfigError::from_kind(ConfigErrorKind::NoSuitableConfigFound(failed_configs)))
            }
        }
    }
}
