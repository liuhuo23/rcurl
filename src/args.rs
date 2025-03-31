use crate::models::Method;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short = 'X', long = "X", value_enum, default_value_t = Method::GET, help="请求方式")]
    pub x: Method,
    #[arg(required = true, value_name = "URL")]
    pub url: String,
    #[arg(short, long, help = "Output file", value_name = "FILE")]
    pub out: Option<String>,
    #[arg(short = 'v', long, help = "启用详细日志输出")]
    pub verbose: bool,
    #[arg(short = 'H', long, help = "设置请求头", value_name = "HEADER")]
    pub headers: Vec<String>,
    #[arg(
        short = 'd',
        long,
        help = "POST请求时，设置请求体",
        value_name = "DATA"
    )]
    pub data: Option<String>,
    #[arg(short = 't', long, help = "设置请求超时时间", value_name = "TIMEOUT")]
    pub timeout: Option<u64>,
    #[arg(
        short = 'c',
        long,
        help = "设置最大重试次数",
        default_value = "4",
        value_name = "RETRY"
    )]
    pub retry: u64,
    #[arg(
        short = 's',
        long,
        help = "设置请求间隔时间",
        default_value = "4",
        value_name = "INTERVAL"
    )]
    pub interval: u64,
}
