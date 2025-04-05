use super::error::RequestError;
use super::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}

impl TryFrom<&str> for HttpVersion {
    type Error = RequestError;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/2.0" => Ok(HttpVersion::Http2_0),
            _ => Err(RequestError::UnsupportedVersion(format!(
                "不支持的HTTP版本: {}",
                value
            ))),
        }
    }
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http1_0 => write!(f, "HTTP/1.0"),
            HttpVersion::Http1_1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2_0 => write!(f, "HTTP/2.0"),
        }
    }
}
