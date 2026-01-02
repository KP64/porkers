//! The porkers library
// # TODO: Documentation

/// DNS Functionality
pub mod dns;

/// Domain Functionality
pub mod domain;

/// General API Information
pub mod general;

/// SSL Functionality
pub mod ssl;

use core::fmt;

use anyhow as _;
use clap as _;
use config as _;
use serde::{Deserialize, Serialize};
use thiserror as _;
use tokio as _;

#[expect(missing_docs, reason = "WIP")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    #[serde(rename = "apikey")]
    pub api_key: String,
    #[serde(rename = "secretapikey")]
    pub secret_api_key: String,
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "api_key: <REDACTED>\nsecret_api_key: <REDACTED>")
    }
}

/// The barebones structure of a status returned by Porkbun
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    /// Any HTTP response code other than 200
    Error,

    /// Status when HTTP response code is 200
    Success,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match *self {
            Self::Error => "ERROR",
            Self::Success => "SUCCESS",
        };

        write!(f, "{status}")
    }
}
