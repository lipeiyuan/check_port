use crate::utils;
use clap::Args;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;

use utils::MAX_TOKEN_LEN;

// Server的子命令
#[derive(Args, Debug)]
#[command(about = "server sub command")]
pub struct Server {
    /// IP address allowed to connect from client
    #[arg(value_parser = utils::parse_ip)]
    #[arg(default_value_t = IpAddr::V4(Ipv4Addr::new(0,0,0,0)))]
    #[arg(long)]
    ip: IpAddr,

    /// udp Port to listen, must be 1 to 65535
    #[arg(value_parser = utils::parse_port)]
    #[arg(long)]
    port: u16,

    /// check token, send from client
    #[arg(default_value_t = super::DEFAULT_TOKEN.to_string())]
    #[arg(long)]
    token: String,
}

pub fn run_server(server: &Server) -> Result<(), String> {
    let token = &server.token;
    let addr = SocketAddr::new(server.ip, server.port);
    println!("begin to bind addr: {}", addr);
    match UdpSocket::bind(addr) {
        Ok(socket) => {
            println!("bind addr: {} success", addr);
            let mut buf = [0u8; MAX_TOKEN_LEN];

            loop {
                let (recv_len, client_addr) = socket.recv_from(&mut buf).expect("recv data failed");
                let recv_buf = &buf[..recv_len];
                let recv_str = str::from_utf8(recv_buf).expect("convert recv buf to str failed");
                println!(
                    "port: {} recv data: {} from addr: {}",
                    server.port, recv_str, client_addr
                );

                if recv_str == token {
                    match socket.send_to(recv_str.as_bytes(), client_addr) {
                        Ok(_) => println!(
                            "port: {} res to addr: {}, data: {} success",
                            server.port, client_addr, recv_str
                        ),

                        Err(_) => eprintln!(
                            "port: {} res to addr: {}, data: {} failed!",
                            server.port, client_addr, recv_str
                        ),
                    }
                } else {
                    eprintln!(
                        "port: {} recv data: [{}] != token: [{}]!",
                        server.port, recv_str, token
                    );
                }
            }
        }
        Err(e) => return Err(format!("bind addr: {} failed, err: {}", addr, e)),
    }
}
