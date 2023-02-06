mod cli;
mod utils;
mod ports;

use std::{net::SocketAddr, ops::Mul, sync::mpsc::channel, time::Duration};

use clap::Parser;
use cli::Cli;
use threadpool::ThreadPool;
use utils::{is_port_open, parse_addresses};

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
    for (ip, addr) in &ips {
        for port in &ports {
            addrs.push((SocketAddr::new(*ip, *port), format!("{}:{}", addr, *port)));
        }
    }
    let count = addrs.len();
    let parallel = count.min(cli.jobs.into());
    let timeout = Duration::from_millis(cli.timeout.into());
    let estimated_millisec = timeout.mul((count / parallel) as u32).as_millis();
    let estimated_sec = if estimated_millisec % 1000 == 0 {
        estimated_millisec / 1000
    } else {
        estimated_millisec / 1000 + 1
    };
    let estimated = if estimated_sec < 60 {
        format!("{estimated_sec}s")
    } else {
        let m = estimated_sec / 60;
        let s = estimated_sec % 60;
        if s == 0 {
            format!("{m}m")
        } else {
            format!("{m}m {s}")
        }
    };

    println!(
        "{}(ips)*{}(ports)={}(addresses), estimated {} to complete\n",
        ips.len(),
        ports.len(),
        count,
        estimated
    );

    let pool = ThreadPool::new(parallel);
    let (tx, rx) = channel();
    for (socket_addr, raw_addr) in addrs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send((raw_addr, is_port_open(&socket_addr, timeout)))
                .unwrap();
        });
    }

    let mut i = 0;
    for (addr, is_open) in rx {
        i += 1;
        if is_open {
            println!("{addr}");
        }
        if i == count {
            break;
        }
    }
}
