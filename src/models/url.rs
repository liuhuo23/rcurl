#[derive(Clone, Debug)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
}

impl Url {
    pub fn addr(&self) -> String {
        let mut addr = self.host.to_string();
        if let Some(port) = self.port {
            addr.push_str(&format!(":{}", port));
        } else {
            addr.push_str(":80");
        }
        addr
    }

    pub fn get_path(&self) -> String {
        let mut res = if self.query.is_some() {
            format!("{}?{}", self.path, self.query.as_ref().unwrap())
        } else {
            self.path.clone()
        };
        if res.is_empty() {
            res = "/".to_string();
        }
        res
    }
}

impl From<&str> for Url {
    fn from(value: &str) -> Self {
        let mut scheme = String::new();
        let mut host;
        let mut port = None;
        let mut path = String::new();
        let mut query = None;

        if let Some(pos) = value.find("://") {
            scheme = value[..pos].to_string();
            let rest = &value[pos + 3..];
            if let Some(pos) = rest.find('/') {
                host = rest[..pos].to_string();
                path = rest[pos..].to_string();
            } else {
                host = rest.to_string();
            }
        } else {
            host = value.to_string();
        }

        if let Some(pos) = host.find(':') {
            port = Some(host[pos + 1..].parse().unwrap());
            host = host[..pos].to_string();
        }

        if let Some(pos) = path.find('?') {
            query = Some(path[pos + 1..].to_string());
            path = path[..pos].to_string();
        }

        Url {
            scheme,
            host,
            port,
            path,
            query,
        }
    }
}

impl From<Url> for String {
    fn from(val: Url) -> Self {
        let mut url = format!("{}://{}", val.scheme, val.host);
        if let Some(port) = val.port {
            url.push_str(&format!(":{}", port));
        }
        url.push_str(&val.path);
        if let Some(query) = val.query {
            url.push_str(&format!("?{}", query));
        }
        url
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let url = "http://localhost:8080/test?name=1";
        let parsed_url: Url = url.into();
        assert_eq!(parsed_url.scheme, "http");
        assert_eq!(parsed_url.host, "localhost");
        assert_eq!(parsed_url.port, Some(8080));
        assert_eq!(parsed_url.path, "/test");
        assert_eq!(parsed_url.query, Some("name=1".to_string()));
    }

    #[test]
    fn to_str() {
        let url = Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8080),
            path: "/test".to_string(),
            query: Some("name=1".to_string()),
        };
        let url_str: String = url.into();
        assert_eq!(url_str, "http://localhost:8080/test?name=1".to_string());
    }

    #[test]
    fn test_get_path() {
        let url = "http://localhost:8080/test?name=1";
        let parsed_url: Url = url.into();
        assert!(parsed_url.get_path() == "/test?name=1");
        let url = "http://localhost:8080/test";
        let parsed_url: Url = url.into();
        assert!(parsed_url.get_path() == "/test");
        let url = "http://localhost:8080";
        let parsed_url: Url = url.into();
        assert!(parsed_url.get_path() == "/");
    }
}
