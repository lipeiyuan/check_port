// udp端口验通工具
// peiyuanli 2024-07-24
// Usage: ./check_port -h/--help

mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::process;
use std::str;

// 子命令
#[derive(Subcommand, Debug)]
enum Commands {
    Server(commands::server::Server),
    Client(commands::client::Client),
}

// 命令行参数解析入口
#[derive(Parser, Debug)]
#[command(
    name = "check_port",
    author = "peiyuanli",
    version = "1.0",
    about = "UDP port checker tool, include server and client"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    println!("cli: {:#?}", cli);

    let result = match &cli.command {
        Commands::Client(client) => match commands::client::run_client(client) {
            Ok(miss_port) => {
                eprintln!("miss port: {:#?}", miss_port);
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::Server(server) => commands::server::run_server(server),
    };

    match result {
        Err(e) => {
            eprintln!("run {:?} failed, \n!!!!ERROR!!!!: {:#?}", cli.command, e);
            process::exit(100);
        }
        Ok(_) => {
            println!("run {:?} success.", cli.command);
            process::exit(0);
        }
    }
}
