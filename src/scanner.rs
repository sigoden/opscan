use std::{net::SocketAddr, time::Duration};

use futures::{stream::FuturesUnordered, StreamExt};
use tokio::net::TcpStream;

use crate::ports::NAMP_TOP_PORTS;

pub struct Scanner<'a> {
    addrs: &'a [(SocketAddr, String)],
    timeout: Duration,
    concurrent: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(addrs: &'a [(SocketAddr, String)], timeout: Duration, concurrent: usize) -> Self {
        Self {
            addrs,
            timeout,
            concurrent,
        }
    }

    pub async fn run(&self) {
        let mut max_addr_len = 0;
        let mut max_port_len = 0;
        for (socket_addr, raw_addr) in self.addrs.iter() {
            max_port_len = max_port_len.max(socket_addr.port().to_string().len());
            max_addr_len = max_addr_len.max(raw_addr.len());
        }
        let mut addr_iter = self.addrs.iter();
        let mut ftrs = FuturesUnordered::new();

        for _ in 0..self.concurrent {
            if let Some((socket_addr, raw_addr)) = addr_iter.next() {
                ftrs.push(self.scan_addr(socket_addr, raw_addr, max_addr_len, max_port_len))
            } else {
                break;
            }
        }

        while (ftrs.next().await).is_some() {
            if let Some((socket_addr, raw_addr)) = addr_iter.next() {
                ftrs.push(self.scan_addr(socket_addr, raw_addr, max_addr_len, max_port_len))
            }
        }
    }

    pub async fn scan_addr(
        &self,
        socket_addr: &SocketAddr,
        raw_addr: &str,
        max_addr_len: usize,
        max_port_len: usize,
    ) {
        if self.connect_addr(socket_addr).await.is_ok() {
            let port = socket_addr.port();
            let name = NAMP_TOP_PORTS.get(&port).unwrap_or(&"unknown");
            println!("{raw_addr:max_addr_len$} {port:<max_port_len$} {name}");
        }
    }

    pub async fn connect_addr(&self, addr: &SocketAddr) -> tokio::io::Result<TcpStream> {
        tokio::time::timeout(self.timeout, async move { TcpStream::connect(addr).await }).await?
    }
}
