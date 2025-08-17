//! Code generator(s) for the `aws-policy-processor` tools.
//!
//! Consumes the outputs from the AWS Service Reference API endpoints, and builds structures
//! that can be used by the policy processor at compile time, rather than having to parse/process
//! the data on every run.

use std::path::{Path, PathBuf};

use aws_service_reference_types::Service;
use color_eyre::eyre;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use syn::Ident;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("failed to read {path}")]
    IO {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("parse error")]
    Parse {
        #[from]
        error: serde_json::Error,
    },
}

pub struct ServiceReferenceLoader {
    path: PathBuf,
}

impl ServiceReferenceLoader {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    pub fn for_file<P: AsRef<Path>>(&self, path: P) -> Result<Service, LoadError> {
        let file = std::fs::File::open(path.as_ref()).map_err(|source| LoadError::IO {
            path: path.as_ref().to_path_buf(),
            source,
        })?;

        serde_json::from_reader(file).map_err(LoadError::from)
    }

    pub fn for_service<S: AsRef<str>>(&self, service: S) -> Result<Service, LoadError> {
        let service_path = self.path.join(format!("{}.json", service.as_ref()));

        self.for_file(service_path)
    }

    fn for_json<S: AsRef<str>>(&self, json: S) -> Result<Service, LoadError> {
        serde_json::from_str(json.as_ref()).map_err(LoadError::from)
    }

    pub fn load_all(&self) -> Result<Vec<Service>, LoadError> {
        let entries = std::fs::read_dir(&self.path).map_err(|source| LoadError::IO {
            path: self.path.clone(),
            source,
        })?;

        let result = entries
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|d| match d {
                Ok(d) => std::fs::read_to_string(d.path()).map_err(|source| LoadError::IO {
                    path: d.path().to_path_buf(),
                    source,
                }),
                Err(source) => Err(LoadError::IO {
                    path: self.path.clone(),
                    source,
                }),
            });
        let result = result
            .flat_map(|value| value.map(|json| self.for_json(&json)))
            .collect::<Vec<Result<_, _>>>();
        eprintln!("Finished parsing service definitions");
        result.into_iter().collect()
    }
}

pub struct Generator {
    loader: ServiceReferenceLoader,
}

impl Generator {
    pub fn new<P: Into<PathBuf>>(data_path: P) -> Self {
        Self {
            loader: ServiceReferenceLoader::new(data_path),
        }
    }

    pub fn generate_action_list(&self) -> eyre::Result<String> {
        let mut buf: Vec<TokenStream> = Vec::new();
        for service in self.loader.load_all()? {
            buf.push(self.generate_actions_for_service(&service));
            eprintln!(" done");
        }

        let code = quote! {
            mod actions {
                #( #buf )*
            }
        };

        Ok(format!("{code}"))
    }

    /// Generate an enum for the given service, for example
    /// ```rust
    /// enum Ec2Actions {
    ///   DescribeInstances,
    ///   //...
    /// }
    ///
    /// impl Display for Ec2Actions { ... }
    /// ```
    pub(crate) fn generate_actions_for_service(&self, service: &Service) -> TokenStream {
        let service_name = service.name.clone();
        eprint!(
            "generating actions for service {service_name}, will generate enum {}...",
            fix_case(&service_name)
        );
        let enum_name = format_ident!("{}Actions", fix_case(&service_name));

        let actions_with_idents = service
            .actions
            .iter()
            .map(|action| (action, fix_ident_name(&action.name)));

        let action_idents = actions_with_idents.clone().map(|e| e.1);
        let enum_code = quote! {
            enum #enum_name {
                #(#action_idents),*
            }
        };

        let display_lines = actions_with_idents.map(|(action, ident)| {
            let action_name = &action.name;
            let value = format!("{service_name}:{action_name}");
            let write_call = quote! { write!(f, #value) };

            quote! {
                #enum_name::#ident => #write_call,
            }
        });
        let display_impl = quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#display_lines)*
                    }
                }
            }
        };

        quote! {
            #enum_code
            #display_impl
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        let data_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("parent should exist")
            .join("service-reference-data");
        Self::new(data_path)
    }
}

/// Fix the service name casing for use in Identifier / enum names
fn fix_case<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut buf = String::new();
    let mut capitalize = true;
    for ch in s.chars() {
        if ch == '-' {
            capitalize = true;
        } else if capitalize {
            buf.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            buf.push(ch);
        }
    }
    buf
}

fn fix_ident_name<S: AsRef<str>>(s: S) -> Ident {
    let s = s.as_ref();
    let s = s.replace("-", "");
    format_ident!("{s}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_service_reference_types::*;

    #[test]
    fn test_gen_enum() {
        let service = Service {
            name: "unknown-service".into(),
            actions: vec![
                Action {
                    name: "PutValue".into(),
                    action_condition_keys: vec![],
                    annotations: Annotations::default(),
                    resources: vec![],
                    supported_by: SupportedBy::default(),
                },
                Action {
                    name: "GetValue".into(),
                    action_condition_keys: vec![],
                    annotations: Annotations::default(),
                    resources: vec![],
                    supported_by: SupportedBy::default(),
                },
            ],
            condition_keys: vec![],
            resources: vec![],
            version: "1.0".into(),
        };

        let generator = Generator::default();
        let code = generator.generate_actions_for_service(&service);
        let code = format!("{code}");
        let file = prettyplease::unparse(&syn::parse_file(&code).unwrap());
        eprintln!("{file}");
    }
}
