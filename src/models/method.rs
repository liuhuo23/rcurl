use clap::{ValueEnum, builder::PossibleValue};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PATCH => write!(f, "PATCH"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::TRACE => write!(f, "TRACE"),
        }
    }
}

impl ValueEnum for Method {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::GET,
            Self::POST,
            Self::PUT,
            Self::DELETE,
            Self::PATCH,
            Self::HEAD,
            Self::OPTIONS,
            Self::CONNECT,
            Self::TRACE,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::GET => PossibleValue::new("GET").help("获取资源"),
            Self::POST => PossibleValue::new("POST").help("创建资源或提交数据"),
            Self::PUT => PossibleValue::new("PUT").help("替换整个资源"),
            Self::DELETE => PossibleValue::new("DELETE").help("删除资源"),
            Self::PATCH => PossibleValue::new("PATCH").help("部分更新资源"),
            Self::HEAD => PossibleValue::new("HEAD").help("获取响应头"),
            Self::OPTIONS => PossibleValue::new("OPTIONS").help("获取服务器支持的HTTP方法"),
            Self::CONNECT => PossibleValue::new("CONNECT").help("建立隧道连接"),
            Self::TRACE => PossibleValue::new("TRACE").help("回显服务器收到的请求"),
        })
    }
}
