use anyhow::Result;
use pistol;
use pistol::scan::TargetScanStatus;
use pistol::Host;
use pistol::Host6;
use pistol::Target;
use std::net::IpAddr;
use std::time::Duration;

use crate::utils::target_addr_parser;
use crate::utils::target_port_parser;

pub fn sysportscan(addr: &str, port: &str, threads_num: usize, timeout: f32) -> Result<()> {
    let addr_vec = target_addr_parser(addr)?;
    let port_vec = target_port_parser(port)?;
    let timeout = Duration::from_secs_f32(timeout);
    let ret = if addr.contains(":") {
        // ipv6
        let mut host_vec = Vec::new();
        for addr in addr_vec {
            match addr {
                IpAddr::V6(addr) => {
                    let host = Host6::new(addr, Some(port_vec.clone()));
                    host_vec.push(host);
                }
                _ => (),
            }
        }
        let target = Target::new6(host_vec);
        pistol::tcp_syn_scan6(target, None, None, threads_num, Some(timeout))?
    } else {
        // ipv4
        let mut host_vec = Vec::new();
        for addr in addr_vec {
            match addr {
                IpAddr::V4(addr) => {
                    let host = Host::new(addr, Some(port_vec.clone()));
                    host_vec.push(host);
                }
                _ => (),
            }
        }
        let target = Target::new(host_vec);
        pistol::tcp_syn_scan(target, None, None, threads_num, Some(timeout))?
    };


    for (ip, s) in ret.results {
        for (port , status) in s.status {
            let port_status = match status {
                TargetScanStatus::Open => "open",
                TargetScanStatus::OpenOrFiltered => "open|filtered",
                _ => "",
            };
            if port_status.len() > 0 {
                println!("{}:{} - {}", ip, port, port_status);
            }
        }
    }


    Ok(())
}
