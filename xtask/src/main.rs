use std::path::PathBuf;
use std::sync::LazyLock;

use clap::Parser;
use color_eyre::eyre::{self, OptionExt, WrapErr};
use color_eyre::owo_colors::OwoColorize;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(clap::Parser)]
enum Flags {
    SyncActions(SyncActions),
}

/// Redownload and update the reference data files.
#[derive(clap::Args, Debug)]
struct SyncActions {
    /// include verbose logging
    #[arg(long, short)]
    verbose: bool,

    /// path to output service reference data, defaults to './service-reference-data'
    #[arg(long, short)]
    path: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let flags = Flags::parse();
    match flags {
        Flags::SyncActions(sync_actions) => sync_actions.run()?,
    }

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    service: String,
    url: String,
}

fn http_client() -> &'static Client {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .expect("couldn't create http client")
    });

    &*CLIENT
}

impl SyncActions {
    fn run(self) -> eyre::Result<()> {
        let output_path = self.path.as_ref().cloned().unwrap_or_else(|| {
            PathBuf::new()
                .join(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("service-reference-data")
        });
        match (output_path.exists(), output_path.is_dir()) {
            (true, false) => {
                return Err(eyre::eyre!(
                    "{} is an existing file",
                    output_path.to_string_lossy()
                ));
            }
            (false, _) => {
                std::fs::create_dir(&output_path).wrap_err("couldn't create output directory")?
            }
            (true, true) => {
                // using existing directory, nothing to do
                ()
            }
        };
        let services = self.fetch_service_reference()?;
        let service_width = services
            .iter()
            .map(|s| s.service.len())
            .max()
            .ok_or_eyre("expected at least one service")?;
        for service in services {
            if self.verbose {
                println!(
                    "{:>service_width$}: fetching {}",
                    service.service.green(),
                    service.url
                );
            }
            let service_details = self.fetch_actions_for_service(&service)?;
            let service_path = output_path.join(format!("{}.json", service.service));
            std::fs::write(service_path, service_details)?;
        }

        Ok(())
    }

    fn fetch_service_reference(&self) -> eyre::Result<Vec<Service>> {
        let client = http_client();
        let resp = client
            .get("https://servicereference.us-east-1.amazonaws.com/")
            .header("Accept", "*/*")
            .send()?
            .error_for_status()?;

        let output: Vec<Service> = serde_json::from_reader(resp)?;
        Ok(output)
    }

    /// Download, but don't deserialize/process the output of the given service reference. The processing will happen elsewhere
    fn fetch_actions_for_service(&self, service: &Service) -> eyre::Result<String> {
        let client = http_client();
        let resp = client
            .get(&service.url)
            .header("Accept", "*/*")
            .send()?
            .error_for_status()?;

        Ok(resp.text()?)
    }
}
