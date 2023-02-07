mod addresses;
mod cli;
mod ports;
mod scanner;

use std::{net::SocketAddr, time::Duration};

use addresses::parse_addresses;
use clap::Parser;
use cli::Cli;
use ports::{FULL_PORTS, TOP1000_PORTS};

fn main() {
    let cli = Cli::parse();
    let default_addrs = vec!["127.0.0.1".into()];
    let addrs = if cli.addresses.is_empty() {
        &default_addrs
    } else {
        &cli.addresses
    };
    let (ips, private) = parse_addresses(addrs);

    if ips.is_empty() {
        println!("error: No IPs could be resolved, aborting scan.");
        std::process::exit(1);
    }
    let ports = if cli.ports.is_empty() {
        if private {
            FULL_PORTS.to_vec()
        } else {
            TOP1000_PORTS.to_vec()
        }
    } else {
        let mut ports: Vec<u16> = cli.ports.iter().flat_map(|v| v.values()).collect();
        ports.dedup();
        ports
    };

    let mut addrs: Vec<(SocketAddr, String)> = vec![];

    for (ip, addr) in &ips {
        for port in &ports {
            addrs.push((SocketAddr::new(*ip, *port), addr.to_string()));
        }
    }

    let timeout = match (cli.timeout, private) {
        (Some(v), _) => v,
        (None, true) => 1000,
        (None, false) => 3000,
    };
    let timeout = Duration::from_millis(timeout as u64);

    let count = addrs.len();
    let concurrency = match (cli.concurrency, private) {
        (Some(v), _) => v,
        (None, true) => 65535,
        (None, false) => 4096,
    };

    let concurrency = (concurrency as usize).min(count);

    let scanner = scanner::Scanner::new(&addrs, timeout, concurrency);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(scanner.run());
}
