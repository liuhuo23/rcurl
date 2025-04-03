pub mod client;
mod dns;
mod headers;
mod method;
mod request;
pub mod url;
mod utils;

pub use dns::resolve_domain;
pub use headers::Headers;
pub use method::Method;
pub use request::Request;
