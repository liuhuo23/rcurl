use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

// DNS 响应码
enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameEroor = 3,
    NotImplemented = 4,
    Refused = 5,
}

// 创建随机查询 ID
fn random_id() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_secs() as u16) ^ (now.subsec_nanos() as u16)
}

// 构建DNS查询包
fn build_query(domain: &str, id: u16) -> Vec<u8> {
    let mut query = Vec::new();

    // 头部
    query.extend_from_slice(&id.to_be_bytes()); // ID
    query.extend_from_slice(&[0x01, 0x00]); // 标志：标准查询，递归期望
    query.extend_from_slice(&[0x00, 0x01]); // 问题数量：1
    query.extend_from_slice(&[0x00, 0x00]); // 回答数量：0
    query.extend_from_slice(&[0x00, 0x00]); // 权威回答数量：0
    query.extend_from_slice(&[0x00, 0x00]); // 附加记录数量：0

    // 问题部分 - 域名
    for part in domain.split('.') {
        query.push(part.len() as u8);
        query.extend_from_slice(part.as_bytes());
    }
    query.push(0x00); // 域名结束标记

    // 问题部分 - 查询类型和类
    query.extend_from_slice(&[0x00, 0x01]); // 类型：A记录
    query.extend_from_slice(&[0x00, 0x01]); // 类：IN (Internet)

    query
}

// 解析DNS响应
fn parse_response(response: &[u8], expected_id: u16) -> Result<Vec<Ipv4Addr>, String> {
    if response.len() < 12 {
        return Err("响应太短".to_string());
    }

    // 解析ID
    let id = u16::from_be_bytes([response[0], response[1]]);
    if id != expected_id {
        return Err(format!("ID不匹配：期望 {}, 收到 {}", expected_id, id));
    }

    // 解析标志
    let flags = u16::from_be_bytes([response[2], response[3]]);
    let qr = (flags >> 15) & 0x1; // 响应位
    let opcode = (flags >> 11) & 0xF; // 操作码
    let rcode = flags & 0xF; // 响应码

    if qr != 1 {
        return Err("不是响应包".to_string());
    }

    if opcode != 0 {
        return Err(format!("未知操作码: {}", opcode));
    }

    match rcode {
        0 => {} // 没有错误
        1 => return Err("DNS格式错误".to_string()),
        2 => return Err("DNS服务器失败".to_string()),
        3 => return Err("域名不存在".to_string()),
        4 => return Err("DNS服务器不支持请求类型".to_string()),
        5 => return Err("DNS查询被拒绝".to_string()),
        _ => return Err(format!("未知DNS错误码: {}", rcode)),
    }

    // 获取问题和回答数量
    let qdcount = u16::from_be_bytes([response[4], response[5]]);
    let ancount = u16::from_be_bytes([response[6], response[7]]);

    if ancount == 0 {
        return Err("没有找到任何答案".to_string());
    }

    // 跳过问题部分
    let mut pos = 12;
    for _ in 0..qdcount {
        // 跳过域名
        while pos < response.len() {
            let len = response[pos] as usize;
            if len == 0 {
                pos += 1;
                break;
            }

            // 检查是否是压缩指针
            if (len & 0xC0) == 0xC0 {
                pos += 2;
                break;
            }

            pos += len + 1;
        }

        // 跳过类型和类
        pos += 4;
    }

    // 解析回答部分
    let mut ips = Vec::new();
    for _ in 0..ancount {
        // 跳过域名（可能是压缩指针）
        if (response[pos] & 0xC0) == 0xC0 {
            pos += 2;
        } else {
            while pos < response.len() {
                let len = response[pos] as usize;
                if len == 0 {
                    pos += 1;
                    break;
                }
                pos += len + 1;
            }
        }

        // 读取记录类型
        if pos + 10 > response.len() {
            return Err("响应数据不完整".to_string());
        }

        let rec_type = u16::from_be_bytes([response[pos], response[pos + 1]]);
        let data_len = u16::from_be_bytes([response[pos + 8], response[pos + 9]]) as usize;

        pos += 10;

        // 检查是否是A记录
        if rec_type == 1 && data_len == 4 && pos + 4 <= response.len() {
            let ip = Ipv4Addr::new(
                response[pos],
                response[pos + 1],
                response[pos + 2],
                response[pos + 3],
            );
            ips.push(ip);
        }

        pos += data_len;
    }

    if ips.is_empty() {
        Err("未找到IPv4地址".to_string())
    } else {
        Ok(ips)
    }
}

// 获取系统的 DNS 服务器地址
fn get_system_dns_servers() -> Result<Vec<String>, String> {
    let mut servers = Vec::new();
    let file =
        File::open("/etc/resolv.conf").map_err(|e| format!("无法打开 /etc/resolv.conf: {}", e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("读取 /etc/resolv.conf 失败: {}", e))?;
        if line.starts_with("nameserver") {
            if let Some(server) = line.split_whitespace().nth(1) {
                servers.push(server.to_string());
            }
        }
    }

    if servers.is_empty() {
        Err("未找到任何 DNS 服务器".to_string())
    } else {
        Ok(servers)
    }
}

// 从 hosts 文件中解析域名
fn resolve_from_hosts(domain: &str) -> Result<Option<Vec<Ipv4Addr>>, String> {
    let file = File::open("/etc/hosts").map_err(|e| format!("无法打开 /etc/hosts: {}", e))?;
    let reader = BufReader::new(file);
    let mut hosts_map: HashMap<String, Vec<Ipv4Addr>> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("读取 /etc/hosts 失败: {}", e))?;
        let line = line.trim();

        // 跳过注释和空行
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        // 解析 IP 和域名
        if let Ok(ip) = parts[0].parse::<Ipv4Addr>() {
            for &host in &parts[1..] {
                hosts_map
                    .entry(host.to_string())
                    .or_insert_with(Vec::new)
                    .push(ip);
            }
        }
    }

    // 查找域名
    Ok(hosts_map.get(domain).cloned())
}

pub fn resolve_domain(domain: &str) -> Result<Vec<Ipv4Addr>, String> {
    // 优先从 hosts 文件中查找
    // if let Some(ips) = resolve_from_hosts(domain)?
    //     .unwrap_or_default()
    //     .into_iter()
    //     .collect::<Vec<_>>()
    // {
    //     return Ok(ips);
    // }
    if let Some(ips) = resolve_from_hosts(domain)? {
        return Ok(ips);
    }

    // 获取系统的 DNS 服务器
    let dns_servers = get_system_dns_servers()?;
    let dns_server = dns_servers.get(0).ok_or("没有可用的 DNS 服务器")?;

    // 创建UDP套接字
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => return Err(format!("绑定套接字失败: {}", e)),
    };

    // 设置超时
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(5))) {
        return Err(format!("设置超时失败: {}", e));
    }

    // 连接到系统的 DNS 服务器
    if let Err(e) = socket.connect(format!("{}:53", dns_server)) {
        return Err(format!("连接DNS服务器失败: {}", e));
    }

    // 生成随机ID
    let query_id = random_id();

    // 构建DNS查询
    let query = build_query(domain, query_id);

    // 发送查询
    if let Err(e) = socket.send(&query) {
        return Err(format!("发送查询失败: {}", e));
    }

    // 接收响应
    let mut buf = [0u8; 512];
    let size = match socket.recv(&mut buf) {
        Ok(n) => n,
        Err(e) => return Err(format!("接收响应失败: {}", e)),
    };

    // 解析响应
    parse_response(&buf[0..size], query_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reslove_domain() {
        println!("baidu.com:{:#?}", resolve_domain("baidu.com").unwrap());
        println!(
            "fengliuhuo.top: {:#?}",
            resolve_domain("barn.fengliuhuo.top").unwrap()
        );
        println!("myhome.org: {:#?}", resolve_domain("myhome.org").unwrap());
        println!(
            "fengliuhuo.top: {:#?}",
            resolve_domain("fengliuhuo.top").unwrap()
        );
        println!("mylinux.org: {:#?}", resolve_domain("mylinux.org").unwrap());
    }
}
