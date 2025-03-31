mod client;
mod dns;
mod headers;
mod method;
#[cfg(feature = "beta")]
mod request;
#[cfg(feature = "reqwest")]
mod reqwest;

pub use dns::resolve_domain;
pub use headers::Headers;
pub use method::Method;
#[cfg(feature = "reqwest")]
pub use reqwest::budild_request;
#[cfg(feature = "reqwest")]
pub use reqwest::build_request;
