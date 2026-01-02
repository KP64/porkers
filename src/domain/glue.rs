use super::DOMAIN_BASE_URL;
use crate::Credentials;
use core::{
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
use serde::Deserialize;
use vec1::Vec1;

/// Combines the Credentials with a list of Ips
#[derive(Debug, serde::Serialize)]
struct WithIPs {
    /// The [Credentials](crate::Credentials)
    #[serde(flatten)]
    creds: Credentials,

    /// A List of IPs to be operated on
    ips: Vec1<IpAddr>,
}

// TODO: Improve Status by adding messages etc.
/// Wrapper for Statuses as returned by Porkbun to be (De)Serialized
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct StatusResponse {
    /// The porkbun internal status
    pub status: crate::Status,
}

impl fmt::Display for StatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status)
    }
}

/// Combines the status returned with additional
/// data about the glue subdomains.
#[derive(Debug, Deserialize)]
pub struct StatusWithHostsResponse {
    /// The glue subdomain with its corresponding IPs
    pub hosts: Vec<(String, IPsResponse)>,

    /// Porkbun returned status
    #[serde(flatten)]
    pub status: StatusResponse,
}

impl fmt::Display for StatusWithHostsResponse {
    // TODO: Refactor according to https://github.com/rust-lang/rust-clippy/issues/15224
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hosts_with_ips = self
            .hosts
            .iter()
            .map(|(host, ips)| {
                let ip4 = ips.v4.as_ref().map(|ipv4s| {
                    ipv4s
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let ip6 = ips.v6.as_ref().map(|ipv6s| {
                    ipv6s
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                match (ip4, ip6) {
                    (None, None) => format!("{host}: n/a"),
                    (Some(ip), None) | (None, Some(ip)) => format!("{host}:\n  {ip}"),
                    (Some(ip4), Some(ip6)) => format!("{host}:\n  {ip4}\n  {ip6}"),
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{hosts_with_ips}")
    }
}

/// Wrapper for all IPs returned by Porkbun
#[derive(Debug, Deserialize)]
pub struct IPsResponse {
    /// The IPv4 addresses if there are any
    pub v4: Option<Vec1<Ipv4Addr>>,

    /// The IPv6 addresses if there are any
    pub v6: Option<Vec1<Ipv6Addr>>,
}

impl fmt::Display for IPsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut final_string = String::new();
        if let Some(ref ipv4s) = self.v4 {
            let fmt_ip4 = ipv4s
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ");
            final_string.push_str(&fmt_ip4);
        }
        if let Some(ref ipv6s) = self.v6 {
            final_string.push('\n');
            let fmt_ip6 = ipv6s
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ");
            final_string.push_str(&fmt_ip6);
        }
        write!(f, "{final_string}")
    }
}

/// Create glue record for a domain.
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn create(
    creds: Credentials,
    domain: &str,
    glue_host_subdomain: &str,
    ips: Vec1<IpAddr>,
) -> Result<StatusResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!(
            "{DOMAIN_BASE_URL}/createGlue/{domain}/{glue_host_subdomain}"
        ))
        .json(&WithIPs { creds, ips })
        .send()
        .await?
        .json()
        .await
}

/// Delete glue record for a domain.
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn delete(
    creds: Credentials,
    domain: &str,
    glue_host_subdomain: &str,
) -> Result<StatusResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!(
            "{DOMAIN_BASE_URL}/deleteGlue/{domain}/{glue_host_subdomain}"
        ))
        .json(&creds)
        .send()
        .await?
        .json()
        .await
}

/// Get all glue records of a domain
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn get(
    creds: &Credentials,
    domain: &str,
) -> Result<StatusWithHostsResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{DOMAIN_BASE_URL}/getGlue/{domain}"))
        .json(&creds)
        .send()
        .await?
        .json()
        .await
}

/// Update glue record for a domain.
#[expect(clippy::missing_errors_doc, reason = "WIP")]
pub async fn update(
    creds: Credentials,
    domain: &str,
    glue_subdomain: &str,
    ips: Vec1<IpAddr>,
) -> Result<StatusResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!(
            "{DOMAIN_BASE_URL}/updateGlue/{domain}/{glue_subdomain}"
        ))
        .json(&WithIPs { creds, ips })
        .send()
        .await?
        .json()
        .await
}
