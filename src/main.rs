//! A client to use the porkbun api

use anyhow::Context as _;
use clap::{Args, Parser, Subcommand};
use config::{Config, File};
use core::net::IpAddr;
use porkers::{Credentials, domain::glue, general};
use reqwest as _;
use serde as _;
use std::{
    io::{self, Write as _},
    path::{Path, PathBuf},
};
use thiserror as _;
use vec1::Vec1;

/// Clap argument for Domain.
/// This allows us to check for valid Domains
#[derive(Args, Debug, Clone)]
struct Domain {
    /// The actual domain used
    #[arg(long)]
    domain: String,
}

/// Clap argument to require at least one ip to be provided.
/// This is especially useful to fulfill the invariant for [Vec1]
#[derive(Args, Debug, Clone)]
struct IPs {
    /// IPs passed into by the cli
    #[arg(long, required = true)]
    ips: Vec<IpAddr>,
}

#[derive(Args, Debug, Clone)]
struct CredentialsArg {
    /// The full path to the file that deserializes into the credentials
    #[arg(long, value_name = "FILE")]
    credential_path: PathBuf,
}

#[derive(Parser)]
#[command(version, about,long_about = None)]
enum Cli {
    /// Command for misc operations
    General {
        /// Command to perform misc tasks
        #[command(subcommand)]
        subcommand: GeneralCmd,
    },
    /// Main command to interact with Glue records
    Glue {
        /// Path to the credentials
        #[command(flatten)]
        credential_path: CredentialsArg,

        /// The domain that will be affected by the [subcommand](Cli::Glue::subcommand)
        #[command(flatten)]
        domain: Domain,

        /// Command to perform on the Glue record
        #[command(subcommand)]
        subcommand: GlueCmd,
    },
}

#[derive(Subcommand, Debug)]
enum GeneralCmd {
    /// Get the pricing listing of all TLDs
    TLDPricing,
}

#[derive(Subcommand, Debug)]
enum GlueCmd {
    /// Create a new Glue record
    Create {
        /// The new subdomain the NS should live under
        #[command(flatten)]
        glue_host_subdomain: Domain,

        /// IPs to be associated with the [NS](GlueCmd::Create::glue_host_subdomain)
        #[command(flatten)]
        ips: IPs,
    },

    /// Delete an existing Glue record
    Delete {
        /// Subdomain of the NS to delete
        #[command(flatten)]
        glue_host_subdomain: Domain,
    },

    /// Get all glue records of the domain
    Get,

    /// Update an existing Glue record
    Update {
        /// Subdomain of the NS
        #[command(flatten)]
        glue_host_subdomain: Domain,

        /// IPs to replace the current ones for the [NS](GlueCmd::Update::glue_host_subdomain)
        #[command(flatten)]
        ips: IPs,
    },
}

/// Takes in the path of the file containing the [Credentials].
/// The file can be of any type as long as its processable by the [config] crate.
#[expect(clippy::missing_errors_doc, reason = "WIP")]
fn parse_credentials_from_file(credential_file_path: &Path) -> anyhow::Result<Credentials> {
    Ok(Config::builder()
        .add_source(File::from(credential_file_path))
        .build()?
        .try_deserialize::<Credentials>()?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // TODO: This should only be done in a verbose mode.
    //       Also allow a custom outfile or something
    let mut stdout = io::stdout();

    match cli {
        Cli::General { subcommand } => match subcommand {
            GeneralCmd::TLDPricing => {
                let pricing = general::get().await?;
                writeln!(stdout, "{pricing}")?;
            }
        },
        Cli::Glue {
            credential_path,
            domain,
            subcommand,
        } => {
            let creds = parse_credentials_from_file(&credential_path.credential_path)?;
            match subcommand {
                GlueCmd::Delete {
                    glue_host_subdomain,
                } => {
                    let resp =
                        glue::delete(creds, &domain.domain, &glue_host_subdomain.domain).await?;
                    writeln!(stdout, "{resp}")?;
                }
                GlueCmd::Get => {
                    let host_with_ips = glue::get(&creds, &domain.domain).await?;
                    writeln!(stdout, "{host_with_ips}")?;
                }
                GlueCmd::Create {
                    glue_host_subdomain,
                    ips,
                } => {
                    let ips = Vec1::try_from_vec(ips.ips)
                        .context("Clap should've guaranteed at least one element for conversion")?;
                    let resp =
                        glue::create(creds, &domain.domain, &glue_host_subdomain.domain, ips)
                            .await?;
                    writeln!(stdout, "{resp}")?;
                }
                GlueCmd::Update {
                    glue_host_subdomain,
                    ips,
                } => {
                    let ips = Vec1::try_from_vec(ips.ips)
                        .context("Clap should've guaranteed at least one element for conversion")?;
                    let resp =
                        glue::update(creds, &domain.domain, &glue_host_subdomain.domain, ips)
                            .await?;
                    writeln!(stdout, "{resp}")?;
                }
            }
        }
    }

    Ok(())
}
