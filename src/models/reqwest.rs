use super::method;
use crate::error::RcurlError;
use anyhow::Result;
use log::{debug, trace};
use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use std::str::FromStr;
use std::time::Duration;

pub fn build_request(
    client: &Client,
    method: method::Method,
    url: &str,
    headers: Option<&Vec<String>>,
    body: Option<&str>,
    timeout: Option<u64>,
) -> Result<RequestBuilder> {
    debug!("开始构造请求: method: {}, url: {}", method, url);
    let mut request = match method {
        method::Method::GET => client.get(url),
        method::Method::POST => client.post(url),
        method::Method::PUT => client.put(url),
        method::Method::DELETE => client.delete(url),
        method::Method::PATCH => client.patch(url),
        method::Method::HEAD => client.head(url),
        method::Method::TRACE => client.request(reqwest::Method::TRACE, url),
        _ => {
            return Err(RcurlError::InvalidMethod(format!("invalid method: {}", method)).into());
        }
    };
    if let Some(headers) = headers {
        trace!("开始解析headers");
        debug!("headers: {:?}", headers);
        let mut head_map = HeaderMap::new();
        for header in headers {
            if let Some((key, value)) = header.split_once(":") {
                if let Ok(value) = HeaderValue::from_str(value) {
                    head_map.insert(reqwest::header::HeaderName::from_str(key.trim())?, value);
                }
            }
        }
        request = request.headers(head_map);
    }
    if let Some(timeout) = timeout {
        request = request.timeout(Duration::from_secs(timeout));
    }
    if let Some(body) = body {
        request = request.body(body.to_string());
    }

    Ok(request)
}
