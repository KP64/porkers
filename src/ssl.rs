use crate::{Credentials, Status};
use core::fmt;
use serde::Deserialize;

/// Wrapper struct for deserialization containing the SSL Bundle
#[derive(Deserialize, Debug)]
pub struct BundleResponse {
    /// The complete certificate chain
    #[serde(rename = "certificatechain")]
    pub certificate_chain: String,

    /// The private key
    #[serde(rename = "privatekey")]
    pub private_key: String,

    /// The public key
    #[serde(rename = "publickey")]
    pub public_key: String,

    /// A status indicating whether or not the command was successfully processed
    pub status: Status,
}

impl fmt::Display for BundleResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Certificate chain: <REDACTED>")?;
        writeln!(f, "Private key: <REDACTED>")?;
        write!(f, "Public key: {}", self.public_key)
    }
}

/// Retrieve the SSL certificate bundle for the domain
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn retrieve_bundle(
    creds: &Credentials,
    domain: &str,
) -> Result<BundleResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!(
            "https://api.porkbun.com/api/json/v3/ssl/retrieve/{domain}"
        ))
        .json(&creds)
        .send()
        .await?
        .json()
        .await
}
