use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("获取http版本失败:{0}")]
    UnsupportedVersion(String),
    #[error("std error: {0}")]
    Io(#[from] io::Error),
    #[error("其他错误:{0}")]
    Other(#[from] anyhow::Error),
    #[error("发送请求失败")]
    SendRquestError(String),
}

pub type Result<T> = std::result::Result<T, RequestError>;
