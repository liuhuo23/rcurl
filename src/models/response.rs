use super::error::RequestError;
use super::error::Result;
use anyhow::Context;
use anyhow::anyhow;
use std::os::macos::raw::stat;
use std::{
    fmt::format,
    fs::read,
    io::{BufRead, BufReader, Read, Result as IoResult, Write},
    net::TcpStream,
};

use super::Headers;

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
                "{}装换失败!",
                value
            ))),
        }
    }
}

const HEADER_END_MARKER: &[u8] = b"\r\n\r\n";

pub struct Response {
    pub headers: Headers,
    pub status: u16,
    pub version: HttpVersion,
    pub body: Vec<u8>,
}

impl Read for Response {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        return Ok(0);
    }
}

impl Write for Response {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.body.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

impl Response {
    // 获取特定头字段
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    // 设置响应状态码
    pub fn set_status(&mut self, status: u16) {
        self.status = status
    }

    // 从原始字节刘解析响应
    pub fn from_bytes(stream: &mut TcpStream) -> Result<Response> {
        let mut reader = BufReader::new(stream);
        // 1. 解析状态行
        let mut status_line = String::new();
        reader.read_line(&mut status_line)?;
        let (version, status) = Response::parse_status_lien(&status_line)?;

        // 2. 解析请求头
        let mut headers = Headers::new();
        let mut header_line = String::new();
        loop {
            header_line.clear();
            reader.read_line(&mut header_line)?;

            if header_line == "\r\n" || header_line.is_empty() {
                break; // 空行表示请求头结束
            }

            if let Some((key, value)) = header_line.split_once(':') {
                headers.set(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(Response {
            headers,
            version,
            status,
            body: Vec::new(),
        })
    }

    pub fn new() -> Self {
        Response {
            headers: Headers::new(),
            status: 200,
            version: HttpVersion::Http1_1,
            body: Vec::new(),
        }
    }

    fn parse_status_lien(line: &str) -> Result<(HttpVersion, u16)> {
        let mut parts = line.split_whitespace();
        let version = parts.next().context("Invalid status line")?;
        let status = parts.next().context("Missing status code")?;
        let status = status.parse::<u16>().context("Invalid status code")?;
        Ok((version.try_into()?, status))
    }
}
