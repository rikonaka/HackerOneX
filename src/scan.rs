use anyhow::Result;
use pistol;
use pistol::Host;
use pistol::Host6;
use pistol::Target;
use std::net::IpAddr;
use std::time::Duration;

use crate::utils::target_addr_parser;
use crate::utils::target_port_parser;

pub fn sysportscan(
    addr: &str,
    port: &str,
    threads_num: usize,
    timeout: f32,
) -> Result<()> {
    let target_addr_vec = target_addr_parser(addr)?;
    let timeout = Duration::from_secs_f32(timeout);
    // let ret = match target_addr {
    //     IpAddr::V4(addr) => {
    //         let host = Host::new(addr, Some(vec![target_port]));
    //         let target = Target::new(vec![host]);
    //         pistol::tcp_syn_scan(target, None, None, threads_num, Some(timeout))?
    //     }
    //     IpAddr::V6(addr) => {
    //         let host = Host6::new(addr, Some(vec![target_port]));
    //         let target = Target::new6(vec![host]);
    //         pistol::tcp_syn_scan6(target, None, None, threads_num, Some(timeout))?
    //     }
    // };

    Ok(())
}
