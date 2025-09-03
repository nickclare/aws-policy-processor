use std::path::PathBuf;

use color_eyre::eyre;
use service_code_generator::Generator;

fn main() -> eyre::Result<()> {
    let output_path = PathBuf::new().join("generated");
    std::fs::create_dir_all(&output_path)?;
    let generator = Generator::new(output_path);
    generator.execute()?;

    Ok(())
}
