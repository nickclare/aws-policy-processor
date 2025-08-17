use color_eyre::eyre;
use service_code_generator::Generator;

fn main() -> eyre::Result<()> {
    let should_format = true;
    let generator = Generator::default();
    let code = generator.generate_action_list()?;

    // let formatted = syn::parse_file(&code).map(|c| prettyplease::unparse(&c))?;
    let final_output = if should_format {
        reformat_code(code)?
    } else {
        code
    };

    println!("{final_output}");
    Ok(())
}

fn reformat_code(code: String) -> eyre::Result<String> {
    let file = syn::parse_file(&code)?;
    Ok(prettyplease::unparse(&file))
}
