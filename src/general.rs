#![expect(clippy::std_instead_of_alloc, reason = "False Positive for BTreeMap")]

use crate::{Credentials, Status};
use core::{fmt, net::IpAddr, num::ParseFloatError};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Wrapper for (De)Serializing all TLD Prices
#[derive(Deserialize, Debug)]
pub struct TLDPricingResponse {
    /// Pricing map of each TLD
    pub pricing: BTreeMap<String, TLDPricing>,

    /// Porkbun returned [Status]
    pub status: Status,
}

impl fmt::Display for TLDPricingResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}", self.status)?;

        for (tld, pricing) in &self.pricing {
            writeln!(f)?;
            writeln!(f, "{tld}:")?;
            writeln!(f, "  Registration: {}", pricing.registration)?;
            writeln!(f, "  Renewal: {}", pricing.renewal)?;
            write!(f, "  Transfer: {}", pricing.transfer)?;
        }
        Ok(())
    }
}

/// Contains the fees for each operation of a TLD
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "TLDPricingWire")]
pub struct TLDPricing {
    /// Registration fee
    pub registration: f32,

    /// Renewal fee
    pub renewal: f32,

    /// Transfer fee
    pub transfer: f32,
}

/// Struct to intermediately store the pricing of a TLD
#[derive(Deserialize)]
struct TLDPricingWire {
    /// Registration fee
    registration: String,

    /// Renewal fee
    renewal: String,

    /// Transfer fee
    transfer: String,
}

impl TryFrom<TLDPricingWire> for TLDPricing {
    type Error = ParseFloatError;

    fn try_from(value: TLDPricingWire) -> Result<Self, Self::Error> {
        let parse_price_to_float = |mut price: String| {
            price.retain(|character| character != ',');
            price.parse()
        };
        Ok(Self {
            registration: parse_price_to_float(value.registration)?,
            renewal: parse_price_to_float(value.renewal)?,
            transfer: parse_price_to_float(value.transfer)?,
        })
    }
}
/// Wrapper struct for (De)Serialization
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PingResponse {
    /// Porkbun returned [Status]
    pub status: Status,

    /// The IP Address of the pinger
    #[serde(rename = "yourIp")]
    pub your_ip: IpAddr,
}

impl fmt::Display for PingResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", self.status)?;
        write!(f, "Your IP: {}", self.your_ip)
    }
}

/// Check default domain pricing information for all supported TLDs. This command does not require authentication.
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn domain_pricing() -> Result<TLDPricingResponse, reqwest::Error> {
    reqwest::get("https://api.porkbun.com/api/json/v3/pricing/get")
        .await?
        .json()
        .await
}

/// The ping endpoint will also return your IP address
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn ping(creds: &Credentials) -> Result<PingResponse, reqwest::Error> {
    reqwest::Client::new()
        .post("https://api.porkbun.com/api/json/v3/ping")
        .json(creds)
        .send()
        .await?
        .json()
        .await
}
