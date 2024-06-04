use anyhow::Ok;
use anyhow::Result;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use subnetwork::CrossIpv4Pool;
use subnetwork::CrossIpv6Pool;
use subnetwork::Ipv4Pool;
use subnetwork::Ipv6Pool;

use crate::errors::TargetAddrParseError;

fn addr_is_ipv4(addr_str: &str) -> bool {
    if addr_str.contains(".") && !addr_str.contains(":") {
        true
    } else {
        false
    }
}

fn addr_is_ipv6(addr_str: &str) -> bool {
    !addr_is_ipv4(addr_str)
}

fn addr_is_subnet(addr_str: &str) -> bool {
    if addr_str.contains("/") {
        true
    } else {
        false
    }
}

fn addr_is_range(addr_str: &str) -> bool {
    if addr_str.contains("-") {
        true
    } else {
        false
    }
}

/// Example: 192.168.1.1/24
fn subnet_addr_parser(addr_str: &str) -> Result<Vec<IpAddr>> {
    // subnet
    let mut ret = Vec::new();
    if addr_is_ipv6(addr_str) {
        // ipv6
        let ipv6_pool = Ipv6Pool::from(addr_str)?;
        for ip in ipv6_pool {
            ret.push(IpAddr::V6(ip));
        }
    } else if addr_is_ipv4(addr_str) {
        // ipv4
        let ipv4_pool = Ipv4Pool::from(addr_str)?;
        for ip in ipv4_pool {
            ret.push(IpAddr::V4(ip));
        }
    } else {
        let error = TargetAddrParseError::new(addr_str);
        return Err(error.into());
    }
    Ok(ret)
}

/// Example: 192.168.1.3-192.168.1.5
fn range_addr_parser(addr_str: &str) -> Result<Vec<IpAddr>> {
    // range
    let mut ret = Vec::new();
    let addr_split: Vec<&str> = addr_str.split("-").collect();
    if addr_split.len() == 2 {
        let start_addr = addr_split[0];
        let end_addr = addr_split[1];
        if addr_is_ipv6(start_addr) && addr_is_ipv6(end_addr) {
            // ipv6
            let start_addr: Ipv6Addr = start_addr.parse()?;
            let end_addr: Ipv6Addr = end_addr.parse()?;
            let cross_ipv6_pool = CrossIpv6Pool::new(start_addr, end_addr)?;
            for ip in cross_ipv6_pool {
                ret.push(IpAddr::V6(ip));
            }
        } else if addr_is_ipv4(start_addr) && addr_is_ipv4(end_addr) {
            // ipv4
            let start_addr: Ipv4Addr = start_addr.parse()?;
            let end_addr: Ipv4Addr = end_addr.parse()?;
            let cross_ipv4_pool = CrossIpv4Pool::new(start_addr, end_addr)?;
            for ip in cross_ipv4_pool {
                ret.push(IpAddr::V4(ip));
            }
        }
    } else {
        let error = TargetAddrParseError::new(addr_str);
        return Err(error.into());
    }
    Ok(ret)
}

fn single_addr_parser(addr_str: &str) -> Result<Option<IpAddr>> {
    let ret = if addr_is_ipv6(addr_str) {
        let addr: Ipv6Addr = addr_str.parse()?;
        Some(IpAddr::V6(addr))
    } else if addr_is_ipv4(addr_str) {
        let addr: Ipv4Addr = addr_str.parse()?;
        Some(IpAddr::V4(addr))
    } else {
        None
    };
    Ok(ret)
}

fn addr_parser(addr_str: &str) -> Result<Vec<IpAddr>> {
    let mut ret = Vec::new();
    if addr_is_subnet(addr_str) {
        let r = subnet_addr_parser(addr_str)?;
        ret.extend(r);
    } else if addr_is_range(addr_str) {
        let r = range_addr_parser(addr_str)?;
        ret.extend(r);
    } else {
        let r = single_addr_parser(addr_str)?;
        match r {
            Some(addr) => ret.push(addr),
            None => return Err(TargetAddrParseError::new(addr_str).into()),
        }
    }
    Ok(ret)
}

/// Legal address format:
/// 192.168.1.1/24
/// 192.168.1.1-192.168.1.3
/// 192.168.1.1,192.168.1.3
/// 192.168.1.1/24,192.168.3.1/24
/// fe80::215:5dff:fe20:b393
/// fe80::215:5dff:fe20:b393-fe80::215:5dff:fe20:b395
/// fe80::215:5dff:fe20:b393,fe80::215:5dff:fe20:b395
pub fn target_addr_parser(addr_str: &str) -> Result<Vec<IpAddr>> {
    let mut ret = Vec::new();
    if addr_str.contains(",") {
        // split it
        let addr_split: Vec<&str> = addr_str.split(",").collect();
        for a in addr_split {
            let r = addr_parser(a)?;
            ret.extend(r);
        }
    } else {
        let r = addr_parser(addr_str)?;
        ret.extend(r);
    }
    Ok(ret)
}

/// Legal port format:
/// 80-90
/// 80,81
pub fn target_port_parser(port_str: &str) -> Result<Vec<u16>> {
    let mut ret = Vec::new();
    if port_str.contains("-") {
        // range port
        let port_split: Vec<&str> = port_str.split("-").collect();
        if port_split.len() == 2 {
            let start_port: u16 = port_split[0].parse()?;
            let end_port: u16 = port_split[1].parse()?;
            if start_port <= end_port {
                for p in start_port..=end_port {
                    ret.push(p);
                }
            }
        }
    } else if port_str.contains(",") {
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_target_addr_parser() {
        let addr_vec = vec![
            "192.168.1.1/24",
            "192.168.1.1-192.168.1.3",
            "192.168.1.1,192.168.1.3",
            "192.168.1.1/24,192.168.3.1/24",
            "fe80::215:5dff:fe20:b393",
            "fe80::215:5dff:fe20:b393-fe80::215:5dff:fe20:b395",
            "fe80::215:5dff:fe20:b393,fe80::215:5dff:fe20:b395",
        ];
        for addr in addr_vec {
            let _r = target_addr_parser(addr).unwrap();
            // println!("{:?}", _r);
        }
    }
}
