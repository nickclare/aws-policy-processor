use std::{fs::File, path::PathBuf};

use aws_service_references::Service;
use color_eyre::eyre::{self, Report};

pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let data_dir = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("service-reference-data");

    let mut errors: Vec<Report> = Vec::new();

    for file in std::fs::read_dir(data_dir)? {
        match file {
            Err(err) => errors.push(Report::new(err).wrap_err(format!("error reading directory"))),
            Ok(entry) => {
                if entry.file_type()?.is_file() {
                    let file = File::open(entry.path())?;
                    let result: Result<Service, serde_json::Error> = serde_json::from_reader(file);
                    if let Err(err) = result {
                        let path = entry.path();
                        let filename = path.file_name().unwrap().to_string_lossy();
                        errors.push(
                            Report::new(err).wrap_err(format!("failed to parse '{}'", filename)),
                        )
                    }
                }
            }
        }
    }

    println!("Encountered {} errors.", errors.len());
    if errors.len() != 0 {
        for e in errors {
            eprintln!("{e:?}");
        }
    }

    Ok(())
}
