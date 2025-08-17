use std::path::PathBuf;

use aws_service_references::Service;
use color_eyre::eyre;

pub fn main() -> eyre::Result<()> {
    let data_path = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("service-reference-data")
        .join("s3.json");

    let data = std::fs::read_to_string(data_path)?;
    let service: Service = serde_json::from_str(&data)?;

    dbg!(service);

    Ok(())
}
