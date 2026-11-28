use quote::{quote};
use proc_macro::TokenStream;

pub(crate) mod util;

mod builtin;

use builtin::*;
use syn::parse_macro_input;

///
/// ```rust, no_run
/// use bash_loadable::prelude::builtin;
/// #[builtin(
/// 
/// )]
/// mod my_builtin {
///     pub fn on_load() -> Result<(), ()> {
///         Ok(())
///     }
///
///     pub fn on_unload() {
///     
///     }
/// 
///     pub fn command(words: Vec<&'a str>) -> BashStatus {
///         
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn builtin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as BuiltinAttrInput);
    let mod_input = parse_macro_input!(item as BuiltinModInput);
    let data = BuiltinData {
        attr: attr_input,
        module: mod_input,
    };
    quote!{ #data }.into()
}
