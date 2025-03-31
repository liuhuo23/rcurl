use super::headers::Headers;

pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub http_version: String,
}

impl Request {
    pub fn to_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(self.method.as_bytes());
        data.extend_from_slice(b" ");
        data.extend_from_slice(self.url.as_bytes());
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request() {
        let mut headers = Headers::new();
        headers.add("Content-Type", "application/json");
        let request = Request {
            url: "http://localhost:8080".to_string(),
            method: "GET".to_string(),
            headers,
            body: Vec::new(),
            http_version: "1.1".to_string(),
        };
        let data = request.to_data();
        assert_eq!(
            data,
            b"GET http://localhost:8080 HTTP/1.1\r\nContent-Type: application/json\r\n\r\n"
        );
    }
}
