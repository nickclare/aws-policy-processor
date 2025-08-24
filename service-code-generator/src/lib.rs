//! Code generator(s) for the `aws-policy-processor` tools.
//!
//! Consumes the outputs from the AWS Service Reference API endpoints, and builds structures
//! that can be used by the policy processor at compile time, rather than having to parse/process
//! the data on every run.

use std::path::PathBuf;

use color_eyre::eyre;
use generator::Module;
use loader::ServiceReferenceLoader;

pub mod generator;
pub mod loader;

pub struct Generator {
    loader: ServiceReferenceLoader,
    output_root: PathBuf,
}

impl Generator {
    pub fn new_with_data_path<P1: Into<PathBuf>, P2: Into<PathBuf>>(
        data_path: P1,
        output_path: P2,
    ) -> Self {
        Self {
            loader: ServiceReferenceLoader::new(data_path),
            output_root: output_path.into(),
        }
    }

    pub fn new<P: Into<PathBuf>>(output_path: P) -> Self {
        let data_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("parent should exist")
            .join("service-reference-data");
        Self::new_with_data_path(data_path, output_path)
    }

    /// Generate everything
    pub fn execute(self) -> eyre::Result<()> {
        let mut root = Module {
            name: "generated".into(),
            code: vec![],
            children: vec![],
        };

        for service in self.loader.load_all()? {
            let service_mod = generator::generate_service_module(service);
            root.children.push(service_mod);
        }

        root.write_to_path(&self.output_root)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_service_reference_types::*;

    // #[test]
    // fn test_gen_enum() {
    //     let service = Service {
    //         name: "unknown-service".into(),
    //         actions: vec![
    //             Action {
    //                 name: "PutValue".into(),
    //                 action_condition_keys: vec![],
    //                 annotations: Annotations::default(),
    //                 resources: vec![],
    //                 supported_by: SupportedBy::default(),
    //             },
    //             Action {
    //                 name: "GetValue".into(),
    //                 action_condition_keys: vec![],
    //                 annotations: Annotations::default(),
    //                 resources: vec![],
    //                 supported_by: SupportedBy::default(),
    //             },
    //         ],
    //         condition_keys: vec![],
    //         resources: vec![],
    //         version: "1.0".into(),
    //     };

    //     let code = generator::generate_actions_enum(&service);
    //     let code = format!("{code}");
    // }
}
