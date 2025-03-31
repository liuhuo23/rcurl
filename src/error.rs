use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcurlError {
    #[error("Invalid method: {0}")]
    InvalidMethod(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
