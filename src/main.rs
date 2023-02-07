mod cli;
mod ports;
mod utils;

use std::{net::SocketAddr, sync::mpsc::channel, time::Duration};

use clap::Parser;
use cli::Cli;
use threadpool::ThreadPool;
use utils::{is_port_open, parse_addresses};

use crate::ports::NAMP_TOP_PORTS;

fn main() {
    let cli = Cli::parse();

    let (ips, _) = parse_addresses(&cli.addresses);

    if ips.is_empty() {
        println!("error: No IPs could be resolved, aborting scan.");
        std::process::exit(1);
    }

    let mut ports: Vec<u16> = cli.ports.iter().flat_map(|v| v.values()).collect();
    ports.dedup();

    let mut addrs: Vec<(SocketAddr, String)> = vec![];
    let mut max_addr_len = 0;
    let max_port_len = ports.iter().max().unwrap().to_string().len();

    for (ip, addr) in &ips {
        max_addr_len = max_addr_len.max(addr.len());
        for port in &ports {
            addrs.push((SocketAddr::new(*ip, *port), addr.to_string()));
        }
    }
    let count = addrs.len();
    let timeout = Duration::from_secs(cli.timeout.into());
    let batch_size = count.min(cli.batch_size.into());
    #[cfg(unix)]
    let batch_size = adjust_batch_size(batch_size);

    let pool = ThreadPool::new(batch_size);
    let (tx, rx) = channel();
    for (socket_addr, raw_addr) in addrs {
        let tx = tx.clone();
        pool.execute(move || {
            let is_open = is_port_open(&socket_addr, timeout);
            tx.send((raw_addr, is_open, socket_addr.port())).unwrap();
        });
    }

    let mut i = 0;
    for (addr, is_open, port) in rx {
        i += 1;
        if is_open {
            let name = NAMP_TOP_PORTS.get(&port).unwrap_or(&"unknown");
            println!("{addr:max_addr_len$} {port:<max_port_len$} {name}");
        }
        if i == count {
            break;
        }
    }
}

#[cfg(unix)]
fn adjust_batch_size(value: usize) -> usize {
    if let Ok((limit, _)) = rlimit::Resource::NOFILE.get() {
        let limit = (limit - 100) as usize;
        if limit < value {
            return limit;
        }
    }
    value
}
