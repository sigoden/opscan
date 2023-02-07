//! Addresses utility

use cidr_utils::cidr::IpCidr;
use lazy_static::lazy_static;
use std::net::{IpAddr, ToSocketAddrs};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

lazy_static! {
    static ref CLASS_A: IpCidr = IpCidr::from_str("10.0.0.0/8").unwrap();
    static ref CLASS_B: IpCidr = IpCidr::from_str("172.16.0.0/12").unwrap();
    static ref CLASS_C: IpCidr = IpCidr::from_str("192.168.0.0/16").unwrap();
    static ref LOOPBACK: IpCidr = IpCidr::from_str("127.0.0.0/8").unwrap();
}

/// Is ip belongs to private network
pub fn is_private_ip(ip: IpAddr) -> bool {
    CLASS_C.contains(ip) || CLASS_B.contains(ip) || CLASS_A.contains(ip) || LOOPBACK.contains(ip)
}

/// Goes through all possible IP inputs (files or via argparsing)
/// Parses the string(s) into IPs
pub fn parse_addresses(addresses: &[String]) -> (Vec<(IpAddr, String)>, bool) {
    let mut ips: Vec<(IpAddr, String)> = Vec::new();
    let backup_resolver =
        Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    for address in addresses {
        let parsed_ips = parse_address(address, &backup_resolver);
        if !parsed_ips.is_empty() {
            ips.extend(parsed_ips);
        }
    }

    let private = ips.iter().all(|(ip, _)| is_private_ip(*ip));

    (ips, private)
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

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_is_private() {
        assert!(is_private_ip(IpAddr::V4(Ipv4Addr::new(172, 20, 1, 1))));
        assert!(is_private_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 8, 1))));
        assert!(is_private_ip(IpAddr::V4(Ipv4Addr::new(10, 0, 1, 1))));
        assert!(is_private_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 1, 1))));
        assert!(!is_private_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
    }
}
