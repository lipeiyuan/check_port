use crate::utils;
use clap::Args;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::{self, sync, sync::Semaphore, task::JoinSet, time::timeout};

// Client的子命令
#[derive(Args, Debug)]
#[command(about = "client sub command")]
pub struct Client {
    /// udp server IP address to connect, for example 127.0.0.1
    #[arg(value_parser = utils::parse_ip)]
    #[arg(long)]
    ip: IpAddr,

    /// udp Port to connect from, must be 1 to 65535
    #[arg(value_parser = utils::parse_port)]
    #[arg(long)]
    from_port: u16,

    /// udp Port to connect to, must be 1 to 65535 and <= from. will connect to IP:[from, to]
    #[arg(value_parser = utils::parse_port)]
    #[arg(long)]
    to_port: u16,

    /// check token, send to server and also check from response
    #[arg(default_value_t = super::DEFAULT_TOKEN.to_string())]
    #[arg(long)]
    token: String,

    /// timeout for check task, ms
    #[arg(default_value_t = 1000)]
    #[arg(long)]
    timeout: u64,

    /// max number of concurrent tasks
    #[arg(default_value_t = 300)]
    #[arg(long)]
    max_task: u16,
}

pub async fn check_port(ip: IpAddr, port: u16, token: &str) -> Result<(), String> {
    let addr = SocketAddr::new(ip, port);

    if port % 10000 == 0 {
        println!("begin to connect addr: {}", addr);
    }

    match utils::bind_addr(&addr).await {
        Ok(socket) => {
            match utils::send_to(&socket, &addr, token).await {
                Ok(_) => {
                    if port % 10000 == 0 {
                        println!("send data: {} to addr: {} success", token, addr);
                    }
                }
                Err(e) => {
                    return Err(format!(
                        "send data: {} to addr: {} failed, err: {}",
                        token, addr, e
                    ));
                }
            }

            match utils::recv_from(&socket, &addr).await {
                Ok(_) => {
                    if port % 10000 == 0 {
                        println!("recv data from addr: {} success", addr);
                    }
                }
                Err(e) => {
                    return Err(format!("recv data from addr: {} failed, err: {}", addr, e));
                }
            }
        }
        Err(e) => {
            return Err(format!("bind addr: {} failed, err: {}", addr, e));
        }
    }

    return Ok(());
}

#[tokio::main]
pub async fn run_client(client: &Client) -> Result<Vec<u16>, String> {
    let from_port = client.from_port;
    let to_port = client.to_port;
    let timeout_ms = client.timeout;

    // 不通的端口vec
    let mutex: Arc<sync::Mutex<Vec<u16>>> = Arc::new(sync::Mutex::new(Vec::new()));
    // tokio task JoinSet，等待所有task完成
    let mut set = JoinSet::new();
    // 信号量，控制最大并发task数（NOTE: 在linux下，并发数量超过200之后差距不大）
    let sem = Arc::new(Semaphore::new(usize::from(client.max_task)));
    // 端口检查超时失败时间ms
    let limit_ms = Duration::from_millis(timeout_ms);

    // 端口范围：[from, to]
    for port in from_port..(to_port + 1) {
        let ip = client.ip.clone();
        let token = client.token.clone();
        let mutex = Arc::clone(&mutex);
        let sem = Arc::clone(&sem);
        set.spawn(async move {
            let permit = sem.acquire_owned().await.expect("acquire sem failed!");
            if let Err(e) = timeout(limit_ms, check_port(ip, port, token.as_str())).await {
                eprintln!("check failed, addr: {} port: {} err: {}", ip, port, e);
                let mut raw = mutex.lock().await;
                (*raw).push(port);
            }
            drop(permit);
        });
    }

    // 等待所有task结束
    while let Some(_) = set.join_next().await {}

    // return前排个序
    let raw = mutex.lock().await;
    return Ok((*raw).clone());
}
