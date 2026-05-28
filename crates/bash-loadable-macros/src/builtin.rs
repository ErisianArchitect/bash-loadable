use std::{borrow::Cow, ffi::{CStr, CString}, sync::LazyLock};

use fancy_regex::Regex;
use proc_macro2::Span;
use syn::{
    Attribute,
    Error as SynError,
    Ident,
    Item,
    ItemConst,
    ItemFn,
    ItemMod,
    LitCStr,
    LitStr,
    Token,
    token::Bracket,
    ext::IdentExt,
    parse::Parse,
    spanned::Spanned,
};

use quote::{
    quote,
    ToTokens,
};

// const SIZES: (usize, usize) = (size_of::<String>(), size_of::<Vec<String>>());

#[derive(Debug, Clone)]
pub enum LongDoc {
    Paragraph(String),
    Paragraphs(Vec<String>),
}

struct Regexes {
    bash_ident: Regex,
    bash_ident_invalid: Regex,
}

static REGEX: LazyLock<Regexes> = LazyLock::new(|| {
    Regexes {
        bash_ident: Regex::new("^[a-zA-Z0-9-_.:]+$").expect("Failed to create bash_ident regex."),
        bash_ident_invalid: Regex::new("[^a-zA-Z0-9-_.:]").expect("Failed to create bash_ident_invalid regex."),
    }
});

impl Parse for LongDoc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let raw = input.parse::<LitStr>()?;
            Ok(Self::Paragraph(raw.value()))
        } else if input.peek(Bracket) {
            let content;
            syn::bracketed!(content in input);
            if content.is_empty() {
                Ok(Self::Paragraph(String::from("")))
            } else {
                let mut strings = Vec::new();
                let next = content.parse::<LitStr>()?;
                strings.push(next.value());
                while !content.is_empty() {
                    _ = content.parse::<Token![,]>()?;
                    if content.is_empty() {
                        break;
                    }
                    let next = content.parse::<LitStr>()?;
                    strings.push(next.value());
                }
                Ok(Self::Paragraphs(strings))
            }
        } else {
            Err(SynError::new(input.span(), "Unexpected input."))
        } 
    }
}

struct BuiltinName {
    pub name: String,
}

impl Parse for BuiltinName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let ident_check = &REGEX.bash_ident_invalid;
            let lit = input.parse::<LitStr>()?;
            let lit_value = lit.value();
            if let Some(cap) = ident_check.find(lit_value.as_str()).expect("Failed to match regex.") {
                let err = format!("Invalid character: '{}' in builtin name.", cap.as_str());
                return Err(SynError::new(lit.span(), err));
            }
            Ok(Self {
                name: lit.value(),
            })
        } else if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            Ok(Self {
                name: ident.to_string(),
            })
        } else {
            Err(SynError::new(input.span(), "Expected either string literal or identifier."))
        }
    }
}

pub struct BuiltinAttrInput {
    pub name: String,
    pub usage_doc: String,
    pub doc: LongDoc,
}

pub struct BuiltinModInput {
    pub module: ItemMod,
    pub has_on_load: bool,
    pub has_on_unload: bool,
}

impl Parse for BuiltinAttrInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(SynError::new(input.span(), "Empty attribute input. Expected `name`, `usage`, and (optional) `doc` arguments."))
        }
        let mut name: Option<String> = None;
        let mut usage_doc: Option<String> = None;
        let mut doc: Option<LongDoc> = None;
        while !input.is_empty() {
            let key = input.parse::<Ident>()?;
            _ = input.parse::<Token![=]>()?;
            let key_str = key.to_string();
            match key_str.as_str() {
                "name" => {
                    if name.is_some() {
                        return Err(SynError::new(key.span(), "Duplicate `name` field."));
                    }
                    name = Some(input.parse::<BuiltinName>()?.name);
                }
                "usage" => {
                    if usage_doc.is_some() {
                        return Err(SynError::new(key.span(), "Duplicate `usage` field."));
                    }
                    usage_doc = Some(input.parse::<LitStr>()?.value());
                }
                "doc" => {
                    if doc.is_some() {
                        return Err(SynError::new(key.span(), "Duplicate `doc` field."));
                    }
                    doc = Some(input.parse::<LongDoc>()?);
                }
                _ => return Err(SynError::new(key.span(), "Unexpected argument.")),
            }
            if !input.is_empty() {
                if input.peek(Token![,]) {
                    _ = input.parse::<Token![,]>()?;
                }
                if input.is_empty() {
                    break;
                }
            } else {
                break;
            }
        }
        match (name, usage_doc) {
            (None, None) => Err(SynError::new(input.span(), "Expected `name` and `usage` arguments.")),
            (None, Some(_)) => Err(SynError::new(input.span(), "Expected `name` argument.")),
            (Some(_), None) => Err(SynError::new(input.span(), "Expected `usage` argument.")),
            (Some(name), Some(usage_doc)) => {
                Ok(Self {
                    name,
                    usage_doc,
                    doc: doc.unwrap_or_else(|| LongDoc::Paragraph(String::from(""))),
                })
            }
        }
    }
}

impl Parse for BuiltinModInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let mut on_load = false;
        let mut on_unload =false;
        let mut command = false;
        match &module.content {
            Some((_, items)) => {
                for item in items.iter() {
                    let Item::Fn(function) = item else {
                        continue;
                    };
                    let name = &function.sig.ident;
                    if name == "on_load" {
                        on_load = true;
                    } else if name == "on_unload" {
                        on_unload = true;
                    } else if name == "command" {
                        command = true;
                    }
                }
            },
            None => todo!(),
        }
        if !command {
            return Err(SynError::new(input.span(), "Missing `command` function."));
        }
        Ok(Self {
            module,
            has_on_load: on_load,
            has_on_unload: on_unload,
        })
    }
}

pub struct BuiltinData {
    pub module: BuiltinModInput,
    pub attr: BuiltinAttrInput,
}

#[must_use]
pub fn to_cstr<'a>(s: &'a str) -> Cow<'a, CStr> {
    if s.is_empty() {
        return Cow::Borrowed(c"");
    }
    match CStr::from_bytes_until_nul(s.as_bytes()) {
        Ok(cstr) => Cow::Borrowed(cstr),
        Err(_) => Cow::Owned(unsafe { CString::new(s).unwrap_unchecked() })
    }
}

fn make_cstr_lit(value: &str) -> LitCStr {
    let cstr = to_cstr(value);
    LitCStr::new(cstr.as_ref(), Span::call_site())
}

impl ToTokens for BuiltinData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut module = self.module.module.clone();
        let Some((_, items)) = &mut module.content else {
            tokens.extend(syn::Error::new(module.span(), "Missing content.").to_compile_error());
            return;
        };
        let long_docs = match &self.attr.doc {
            LongDoc::Paragraph(raw) => vec![quote!( #raw )],
            LongDoc::Paragraphs(paras) => {
                paras.iter().map(|s| quote!( #s )).collect::<Vec<_>>()
            }
        };
        let para_count = long_docs.len();
        let name = self.attr.name.as_str();
        let name_cstr = make_cstr_lit(name);
        let usage = self.attr.usage_doc.as_str();
        let usage_cstr = make_cstr_lit(usage);
        
        let struct_name = format!("{name}_struct");
        let builtin_name = format!("{name}_builtin");

        let on_load_tokens = if self.module.has_on_load {
            let load_name = format!("{builtin_name}_load");
            quote! {
                #[unsafe(export_name = #load_name)]
                pub extern "C" fn buildin_load(s: *const ::core::ffi::c_char) -> c_int {
                    // returns 1 for success, 0 for failure.
                    match self::on_load() {
                        Ok(()) => 1,
                        Err(err) => {
                            err.eprintln();
                            0
                        },
                    }
                }
            }
        } else {
            quote!()
        };

        let on_unload_tokens = if self.module.has_on_unload {
            let unload_name = format!("{builtin_name}_unload");
            quote! {
                #[unsafe(export_name = #unload_name)]
                pub extern "C" fn builtin_unload() {
                    self::on_unload();
                }
            }
        } else {
            quote!()
        };
        
        let bash_loadable = crate::util::get_bash_loadable_name();
        // #[allow(non_upper_case_globals)]
        let ffi_block: syn::Item = syn::parse_quote!(const _: () = {
            use super::*;
            use ::std::ffi::c_int;
            use #bash_loadable::{
                ffi::{
                    builtin::{
                        Builtin,
                        BuiltinInfo,
                        BuiltinFlags,
                    },
                    word::WordList,
                },
                util::{
                    docs::{LongDoc},
                    ffi::{BashStatus, BashStatusResult, BashStatusError},
                },
            };
            #[unsafe(export_name = #builtin_name)]
            pub extern "C" fn builtin_command(word_list: WordList) -> BashStatus {
                let words = word_list.into_string_vec();
                self::command(words.into()).eprintln_then_into_status()
            }

            #on_load_tokens

            #on_unload_tokens

            #[unsafe(export_name = #struct_name)]
            pub static BUILTIN_STRUCT: Builtin = BuiltinInfo {
                name: (#name_cstr).as_ptr(),
                function: builtin_command,
                short_doc: (#usage_cstr).as_ptr(),
                long_doc: {
                    const LONG_DOC: LongDoc<#para_count> = LongDoc::new([
                        #(
                            ::core::concat!(
                                #long_docs,
                                "\0"
                            ).as_ptr().cast::<::core::ffi::c_char>(),
                        )*
                    ]);
                    LONG_DOC.as_ptr()
                },
            }.build();
        };);
        items.push(ffi_block);
        tokens.extend(quote!(const _: () = { #module };));
    }
}
