use std::io::Read;

use aws_policy_processor::{model::PolicyKind, validator::validate_statement};
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let input = read_input()?;
    let statement = serde_json::from_str(&input).into_diagnostic()?;
    validate_statement(&statement, PolicyKind::ServiceControl)
}

fn read_input() -> Result<String> {
    let mut stdin = std::io::stdin().lock();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).into_diagnostic()?;
    Ok(String::from_utf8(buf).into_diagnostic()?)
}
