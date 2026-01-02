/// Glue Records
pub mod glue;

/// URL Forwarding
pub mod url_forward;

/// Nameserver
pub mod ns;

/// The base url on used for all domain related operations
const DOMAIN_BASE_URL: &str = "https://api.porkbun.com/api/json/v3/domain";

#[expect(missing_docs, reason = "WIP")]
pub fn list_all() {
    unimplemented!()
}

#[expect(missing_docs, reason = "WIP")]
pub fn check() {
    unimplemented!()
}
