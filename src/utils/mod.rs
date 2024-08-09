use core::str;
use std::net::{IpAddr, SocketAddr};
use std::ops::RangeInclusive;
use tokio::{self, net::UdpSocket};

// 端口有效范围
const PORT_RANGE: RangeInclusive<usize> = 1..=65535;
// token最大长度
pub const MAX_TOKEN_LEN: usize = 128;

pub async fn bind_addr(addr: &SocketAddr) -> Result<UdpSocket, String> {
    match UdpSocket::bind("0.0.0.0:0").await {
        Err(e) => Err(format!("bind addr: {} failed, err: {}", addr, e)),
        Ok(socket) => Ok(socket),
    }
}

pub async fn send_to(socket: &UdpSocket, addr: &SocketAddr, token: &str) -> Result<(), String> {
    match socket.send_to(token.as_bytes(), addr).await {
        Err(e) => Err(format!(
            "send data: {} to addr: {} failed, err: {}",
            token, addr, e
        )),
        Ok(_) => Ok(()),
    }
}

pub async fn recv_from(socket: &UdpSocket, addr: &SocketAddr) -> Result<String, String> {
    let mut buf = [0u8; MAX_TOKEN_LEN];
    match socket.recv_from(&mut buf).await {
        Err(e) => Err(format!("recv data from addr: {} failed, err: {}", addr, e)),
        Ok(_) => match str::from_utf8(&buf[..]) {
            Err(e) => Err(format!(
                "convert data from addr: {} failed, err: {}",
                addr, e
            )),
            Ok(data) => Ok(data.to_string()),
        },
    }
}

// 解析IP端口
pub fn parse_port(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("port: {} is not a valid port number!", s))?;

    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    } else {
        return Err(format!(
            "port: {} not in valid range {}-{}",
            s,
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ));
    }
}

// 解析IP地址
pub fn parse_ip(s: &str) -> Result<IpAddr, String> {
    let addr = s.parse::<IpAddr>();
    match addr {
        Err(e) => return Err(format!("ip: {} is not valid!  err: {}", s, e)),
        Ok(ip) => return Ok(ip),
    }
}
