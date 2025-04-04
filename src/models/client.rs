use super::error::RequestError;
use super::error::Result;
use super::request::Request;
use super::url::Url;
use super::{Headers, Method};
use crate::models::response::Response;
use anyhow::anyhow;
use log::debug;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct Client {
    stream: Option<TcpStream>,
    timeout: Duration,
    request: Option<Request>,
}

impl Client {
    /// 创建新客户端
    pub fn new() -> Self {
        Client {
            stream: None,
            timeout: Duration::new(20, 0), // 默认超时时间为5秒
            request: None,
        }
    }

    /// 连接到服务器(带5秒超时)
    fn connect<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let addr = addr.to_socket_addrs()?.next().unwrap();
        let stream = TcpStream::connect_timeout(&addr, self.timeout)?;
        self.stream = Some(stream);
        Ok(())
    }

    /// 发送请求
    pub fn send_request(&mut self, url: &str, method: Method) -> Result<Response> {
        let url_ = Url::from(url);
        self.connect(url_.addr())?;
        if self.request.is_none() {
            self.request = Some(Request::build(url, method));
        }
        let host_value = format!("{}", url_.host);
        let request = self.request.as_mut().unwrap();
        request.set("Host".to_string(), host_value);
        self.execute()
    }

    /// 读取响应(使用缓冲区方式)
    fn read_response(&mut self) -> std::io::Result<Vec<u8>> {
        if let Some(stream) = &mut self.stream {
            const BUF_SIZE: usize = 1024;
            let mut buffer = Vec::new();
            let mut chunk = [0u8; BUF_SIZE];
            loop {
                match stream.read(&mut chunk) {
                    Ok(0) => break, // 读取结束
                    Ok(n) => {
                        buffer.extend_from_slice(&chunk[..n]);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                }
            }
            Ok(buffer)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected to server",
            ))
        }
    }

    /// 获取 request
    pub fn get_request(&self) -> Option<&Request> {
        self.request.as_ref()
    }

    /// get请求
    pub fn get(&mut self, url: &str) -> &mut Request {
        self.request = Some(Request::build(url, Method::GET));
        self.request.as_mut().unwrap()
    }

    /// 执行请求
    pub fn execute(&mut self) -> Result<Response> {
        if let Some(request) = self.request.take() {
            let addr = request.addr();
            self.connect(addr)?;
            if let Some(stream) = self.stream.as_mut() {
                let request_bytes = request.to_bytes();
                debug!("Request:\n{}", String::from_utf8_lossy(&request_bytes));
                match stream.write_all(&request_bytes) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(RequestError::SendRquestError(format!("{e}")));
                    }
                };
                self.request = Some(request);
                let mut response = Response::from_bytes(stream)?;
                response.get_body()?;
                Ok(response)
            } else {
                Err(anyhow!("Not connected to server").into())
            }
        } else {
            Err(anyhow!("No request to execute").into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_client_request_response() -> Result<()> {
        let mut client = Client::new();
        client.get("http://www.baidu.com/hello");
        let response = client.execute()?;
        println!("{}", String::from_utf8_lossy(&response.body));
        Ok(())
    }
}
