use super::Headers;
use super::error::Result;
use super::http_version::HttpVersion;
use anyhow::Context;
use log::debug;
use std::{
    io::{BufRead, BufReader, Read, Result as IoResult},
    net::TcpStream,
};

pub struct Response<'a> {
    pub headers: Headers,
    pub status: u16,
    pub version: HttpVersion,
    reader: BufReader<&'a mut TcpStream>,
    pub body: Vec<u8>,
    content_length: Option<u64>,
    content_disposition: Option<String>,
}

impl Read for Response<'_> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        // 如果有content_length，确保不会读取超过指定长度
        if let Some(len) = self.content_length {
            let already_read = self.body.len() as u64;
            if already_read >= len {
                return Ok(0); // 已经读取完毕
            }

            let remaining = len - already_read;
            let to_read = std::cmp::min(buf.len() as u64, remaining) as usize;
            let n = self.reader.read(&mut buf[..to_read])?;
            self.body.extend_from_slice(&buf[..n]);
            return Ok(n);
        }

        // 普通读取
        let n = self.reader.read(buf)?;
        self.body.extend_from_slice(&buf[..n]);
        Ok(n)
    }
}

impl<'a> Response<'a> {
    // 从原始字节流解析响应
    pub fn from_bytes(stream: &'a mut TcpStream) -> Result<Response<'a>> {
        let mut reader = BufReader::new(&mut *stream);
        // 2. 解析请求头
        let mut headers = Headers::new();
        let mut header_line = String::new();
        reader.read_line(&mut header_line)?;
        let (version, status) = Response::parse_status_lien(&header_line)?;
        loop {
            header_line.clear();
            reader.read_line(&mut header_line)?;
            if header_line.is_empty() {
                debug!("Header line is empty, skipping");
                continue;
            }

            if header_line == "\r\n" || header_line.is_empty() {
                break;
            }

            if let Some((key, value)) = header_line.split_once(':') {
                headers.set(key.trim().to_string(), value.trim().to_string());
            }
        }
        debug!("Response Headers:\n{:?}", headers);
        // 解析Content-Length和Content-Disposition头
        let content_length = headers
            .get("Content-Length")
            .and_then(|s| s.parse::<u64>().ok());
        let content_disposition = headers.get("Content-Disposition").map(|s| s.to_string());

        Ok(Response {
            headers,
            version,
            status,
            reader,
            body: Vec::new(),
            content_length,
            content_disposition,
        })
    }

    // 获取响应体数据(惰性加载)
    pub fn get_body(&mut self) -> Result<&[u8]> {
        if self.body.is_empty() {
            match self.content_length {
                Some(len) if len > 0 => {
                    // 精确分配内存
                    self.body.reserve_exact(len as usize);
                    // 精确读取指定长度
                    let mut remaining = len;
                    let mut chunk = vec![0u8; 8192]; // 8KB缓冲区

                    while remaining > 0 {
                        let to_read = std::cmp::min(chunk.len() as u64, remaining) as usize;
                        let bytes_read = self.reader.read(&mut chunk[..to_read])?;
                        if bytes_read == 0 {
                            break;
                        }
                        self.body.extend_from_slice(&chunk[..bytes_read]);
                        remaining -= bytes_read as u64;
                    }

                    if remaining > 0 {
                        return Err(anyhow::anyhow!(
                            "提前到达流结尾，预期读取{}字节，实际读取{}字节",
                            len,
                            len - remaining
                        )
                        .into());
                    }
                }
                _ => {
                    // 如果没有Content-Length，读取直到EOF
                    self.reader.read_to_end(&mut self.body)?;
                }
            }
        }
        Ok(&self.body)
    }

    fn parse_status_lien(line: &str) -> Result<(HttpVersion, u16)> {
        let mut parts = line.split_whitespace();
        let version = parts.next().context("Invalid status line")?;
        let status = parts.next().context("Missing status code")?;
        let status = status.parse::<u16>().context("Invalid status code")?;
        Ok((version.try_into()?, status))
    }
}
