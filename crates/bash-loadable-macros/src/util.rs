use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};

pub fn get_bash_loadable_name() -> syn::Path {
    let found = crate_name("bash-loadable").expect("Failed to get bash-loadable crate name (util.rs using proc_macro_crate).");
    match found {
        FoundCrate::Itself => syn::parse_quote! { crate },
        FoundCrate::Name(name) => {
            let ident = format_ident!("{}", name);
            syn::parse_quote! { ::#ident }
        },
    }
}
