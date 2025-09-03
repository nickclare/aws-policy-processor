use std::path::{Path, PathBuf};

use aws_service_reference_types::Service;
use rayon::prelude::*;
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
