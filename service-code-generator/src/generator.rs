//! Code generator

use std::{collections::BTreeMap, fs, io, path::Path, sync::Arc};

use aws_service_reference_types::{Action, Service};
use color_eyre::owo_colors::OwoColorize;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use syn::Ident;

// Design docs:
//
// - The code we want to generate consists of two main parts:
//   1. Some concrete enums, containing actions, condition keys, etc.
//   2. A number of trait implementations for those enums, once they are generated.
//

/// A generic enum that will be generated.
#[derive(Clone)]
pub struct Enum<V, E> {
    name: String,
    value: Arc<V>,
    entries: BTreeMap<String, Arc<E>>,
}

impl Enum<Service, Action> {
    pub fn actions_enum(service: Arc<Service>) -> Enum<Service, Action> {
        // let name = format_ident!("{}Actions", service.name.to_upper_camel_case());
        let name = format!("{}Actions", service.name.to_upper_camel_case());
        let entries = service
            .actions
            .iter()
            .cloned()
            .map(|action| {
                let action_name = format!("{}", action.name.to_upper_camel_case());
                (action_name, Arc::new(action))
            })
            .collect::<BTreeMap<_, _>>();

        Enum {
            name,
            value: service,
            entries,
        }
    }
}

impl<V, E> CodeBlock for Enum<V, E> {
    fn token_stream(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let entries = self.entries.keys().map(|e| format_ident!("{e}"));
        quote! {
            #[derive(Debug, Clone, Copy, std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
            pub enum #name {
                #(#entries),*
            }
        }
    }
}

pub(crate) trait CodeBlock {
    /// Generate a token stream for this block of code.
    fn token_stream(&self) -> TokenStream;
}

struct DisplayImpl {
    pub(crate) typ: Arc<Enum<Service, Action>>,
}

impl CodeBlock for DisplayImpl {
    fn token_stream(&self) -> TokenStream {
        let enum_name = format_ident!("{}", &self.typ.name);
        let match_elements = self.typ.entries.iter().map(|(ident, action)| {
            let ident = format_ident!("{}", ident);
            let action_name = format!("{}:{}", self.typ.value.name, action.name);
            quote! {
                #enum_name::#ident => write!(f, #action_name),
            }
        });

        quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#match_elements)*
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct WrappedCodeBlock {
    inner: Arc<dyn CodeBlock + Sync + Send>,
}

impl WrappedCodeBlock {
    // pub fn wrap<T>(code: T) -> Self
    // where
    //     T: CodeBlock,
    //     T: Send + Sync + 'static,
    // {
    //     let inner = Arc::new(code);
    //     Self { inner }
    // }

    pub fn new<T>(inner: Arc<T>) -> Self
    where
        T: CodeBlock,
        T: Send + Sync + 'static,
    {
        Self { inner }
    }
}

impl std::ops::Deref for WrappedCodeBlock {
    type Target = dyn CodeBlock;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

/// Rust module that will be generated and output into a file or set of files.
pub struct Module {
    pub(crate) name: String,
    pub(crate) code: Vec<WrappedCodeBlock>,
    pub(crate) children: Vec<Module>,
}

impl Module {
    pub fn write_to_path(&self, base: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(base.as_ref())?;
        let module_path = base.as_ref().join(&self.name);
        let filename = module_path.with_extension("rs");
        let code = self.code.iter().map(|e| e.token_stream());
        let module_decls = self.children.iter().map(|e| {
            let name = format_ident!("{}", e.name);
            quote! { pub mod #name; }
        });

        let content = quote! {
            #(#code)*

            #(#module_decls)*
        };

        let header = "// NOTE: This code is automatically generated - do not update it manually";
        let content = rustfmt_wrapper::rustfmt(content).expect("code should be formattable");
        let content = format!("{header}\n\n{content}");
        fs::write(filename, content)?;

        self.children
            .par_iter()
            .try_for_each(|module| module.write_to_path(&module_path))?;

        Ok(())
    }
}

pub fn generate_service_module(service: Service) -> Module {
    let service = Arc::new(service);
    let name = service.name.to_snake_case();
    let enum_code = Arc::new(Enum::actions_enum(service));
    let display_impl = Arc::new(DisplayImpl {
        typ: Arc::clone(&enum_code),
    });

    Module {
        name,
        code: vec![
            WrappedCodeBlock::new(enum_code),
            WrappedCodeBlock::new(display_impl),
        ],
        children: vec![],
    }
}
