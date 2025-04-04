use super::{Method, headers::Headers, url::Url};

pub struct Request {
    url: Url,
    pub method: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub http_version: String,
}

impl Request {
    pub fn build(url: &str, method: Method) -> Request {
        Request {
            url: url.into(),
            method: method.to_string(),
            headers: Headers::default(),
            body: Vec::new(),
            http_version: "1.1".to_string(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(self.method.as_bytes());
        data.extend_from_slice(b" ");
        data.extend_from_slice(&self.url.get_path().as_bytes());
        data.extend_from_slice(b" HTTP/");
        data.extend_from_slice(self.http_version.as_bytes());
        data.extend_from_slice(b"\r\n");
        for (key, value) in &self.headers {
            data.extend_from_slice(key.as_bytes());
            data.extend_from_slice(b": ");
            data.extend_from_slice(value.as_bytes());
            data.extend_from_slice(b"\r\n");
        }
        data.extend_from_slice(b"\r\n");
        data.extend_from_slice(&self.body);
        data
    }

    pub fn set(&mut self, key: String, value: String) {
        self.headers.set(key, value);
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.body = body.to_vec();
    }

    pub fn addr(&self) -> String {
        format!("{}", self.url.addr())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request() {
        let mut headers = Headers::new();
        headers.add("Content-Type".to_string(), "application/json".to_string());
        headers.add("user_id".to_string(), "1".to_string());
        let request = Request {
            url: "http://localhost:8008".into(),
            method: "GET".to_string(),
            headers,
            body: Vec::new(),
            http_version: "1.1".to_string(),
        };
        let data = request.to_bytes();
        println!("{:?}", String::from_utf8_lossy(&data));
        assert_eq!(
            data,
            b"GET http://localhost:8008 HTTP/1.1\r\nContent-Type: application/json\r\nuser_id: 1\r\n\r\n"
        );
    }
}
