use std::collections::HashMap;

pub enum HeaderKey {
    Accept,
    ContentType,
    UserAgent,
    Authorization,
    Host,
    AcceptCharset,
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
            HeaderKey::AcceptCharset => "Accept-Charset",
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

impl From<HeaderKey> for &str {
    fn from(val: HeaderKey) -> Self {
        val.as_str()
    }
}

#[derive(Clone, Debug)]
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
    /// 添加请求头
    /// 如果存在则不添加
    pub fn add(&mut self, key: String, value: String) {
        self.headers.entry(key).or_insert_with(|| value.to_string());
    }

    /// 不管是否存在都添加请求头
    /// 如果存在则覆盖
    pub fn set(&mut self, key: String, value: String) {
        self.headers.insert(key, value.to_string());
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

impl Default for Headers {
    fn default() -> Self {
        let mut header = Headers::new();
        header.add("User-Agent".to_string(), "rcurl/1.0".to_string());
        header.add("Accept".to_string(), "*/*".to_string());
        header.add("Connection".to_string(), "close".to_string());
        header.add(
            "Accept-Language".to_string(),
            "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7,en-GB;q=0.6".to_string(),
        );
        header.add(HeaderKey::AcceptCharset.into(), "charset=utf8".to_string());
        header
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

#[cfg(test)]
mod tests {
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
        let iter = MyIterator { current: 0, max: 5 };
        for value in iter {
            println!("Value: {}", value);
        }
    }
}
