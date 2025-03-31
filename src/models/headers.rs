use std::collections::HashMap;

pub enum HeaderKey {
    Accept,
    ContentType,
    UserAgent,
    Authorization,
    Host,
}

impl HeaderKey {
    // 将 HeaderKey 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            HeaderKey::Accept => "Accept",
            HeaderKey::ContentType => "Content-Type",
            HeaderKey::UserAgent => "User-Agent",
            HeaderKey::Authorization => "Authorization",
            HeaderKey::Host => "Host",
        }
    }
}

impl From<HeaderKey> for String {
    fn from(key: HeaderKey) -> Self {
        key.as_str().to_string()
    }
}

impl TryFrom<&str> for HeaderKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accept" => Ok(HeaderKey::Accept),
            "Content-Type" => Ok(HeaderKey::ContentType),
            "User-Agent" => Ok(HeaderKey::UserAgent),
            "Authorization" => Ok(HeaderKey::Authorization),
            "Host" => Ok(HeaderKey::Host),
            _ => Err(format!("未知的 HeaderKey: {}", value)),
        }
    }
}

impl Into<&str> for HeaderKey {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.headers.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.headers.values()
    }
    // 添加请求头
    pub fn add(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    // 获取请求头
    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    // 移除请求头
    pub fn remove(&mut self, key: &str) {
        self.headers.remove(key);
    }

    // to_string
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.headers {
            result.push_str(&format!("{}: {}\r\n", key, value));
        }
        result
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

pub struct HeaderBuilder {
    headers: Headers,
}

impl HeaderBuilder {
    pub fn build(key: &str, value: &str) -> Self {
        let mut headers = Headers::new();
        headers.add(key, value);
        HeaderBuilder { headers }
    }

    pub fn insert(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.add(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let mut builder = HeaderBuilder::build(HeaderKey::Accept.into(), "text/html");
        builder
            .insert(HeaderKey::ContentType.into(), "application/json")
            .insert(HeaderKey::UserAgent.into(), "rcurl/1.0");

        let headers = builder.headers.to_string();
        assert!(headers.contains("Accept: text/html"));
        assert!(headers.contains("Content-Type: application/json"));
        assert!(headers.contains("User-Agent: rcurl/1.0"));
        println!("{headers}");
    }

    // 自定义迭代器实现
    struct MyIterator {
        current: usize,
        max: usize,
    }

    impl Iterator for MyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_my_iterator() {
        let mut iter = MyIterator { current: 0, max: 5 };
        while let Some(value) = iter.next() {
            println!("Value: {}", value);
        }
    }
}
