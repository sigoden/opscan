use cidr_utils::cidr::IpCidr;
use std::{
    net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

/// Whether a port is opening
pub fn is_port_open(addr: &SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(addr, timeout).is_ok()
}

/// Goes through all possible IP inputs (files or via argparsing)
/// Parses the string(s) into IPs
pub fn parse_addresses(addresses: &[String]) -> (Vec<(IpAddr, String)>, Vec<&String>) {
    let mut ips: Vec<(IpAddr, String)> = Vec::new();
    let mut unresolved_addresses: Vec<&String> = Vec::new();
    let backup_resolver =
        Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    for address in addresses {
        let parsed_ips = parse_address(address, &backup_resolver);
        if !parsed_ips.is_empty() {
            ips.extend(parsed_ips);
        } else {
            unresolved_addresses.push(address);
        }
    }

    (ips, unresolved_addresses)
}

/// Given a string, parse it as an host, IP address, or CIDR.
/// This allows us to pass files as hosts or cidr or IPs easily
/// Call this everytime you have a possible IP_or_host
fn parse_address(address: &str, resolver: &Resolver) -> Vec<(IpAddr, String)> {
    match IpCidr::from_str(address) {
        Ok(cidr) => cidr.iter().map(|v| (v, v.to_string())).collect(),
        Err(_) => match resolve_ip_from_host(address, resolver) {
            Some(v) => vec![v],
            None => vec![],
        },
    }
}

/// Uses DNS to get the IPS assiocated with host
fn resolve_ip_from_host(source: &str, backup_resolver: &Resolver) -> Option<(IpAddr, String)> {
    if let Ok(mut addrs) = source.to_socket_addrs() {
        if let Some(ip) = addrs.next() {
            return Some((ip.ip(), source.to_string()));
        }
    } else if let Ok(addrs) = backup_resolver.lookup_ip(source) {
        return addrs.iter().next().map(|v| (v, source.to_string()));
    }
    None
}
