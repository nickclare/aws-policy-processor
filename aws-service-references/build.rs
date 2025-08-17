use std::path::PathBuf;

use color_eyre::eyre;
use service_code_generator::Generator;

/// Run generation at build time
fn run() -> eyre::Result<()> {
    let generator = Generator::default();
    let actions = generator.generate_action_list()?;
    let actions = rustfmt_wrapper::rustfmt(actions)?;
    let out_dir =
        PathBuf::new().join(std::env::var("OUT_DIR").expect("Cargo should set OUT_DIR envvar"));

    std::fs::write(out_dir.join("generated_actions.rs"), actions.as_bytes())?;

    println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}

fn main() {
    run().unwrap();
}
