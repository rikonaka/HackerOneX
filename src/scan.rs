use anyhow::Result;
use pistol::os_detect;
use pistol::os_detect6;
use pistol::scan::TargetScanStatus;
use pistol::tcp_ack_scan;
use pistol::tcp_ack_scan6;
use pistol::tcp_connect_scan;
use pistol::tcp_connect_scan6;
use pistol::tcp_fin_scan;
use pistol::tcp_fin_scan6;
use pistol::tcp_maimon_scan;
use pistol::tcp_maimon_scan6;
use pistol::tcp_null_scan;
use pistol::tcp_null_scan6;
use pistol::tcp_syn_scan;
use pistol::tcp_syn_scan6;
use pistol::tcp_window_scan;
use pistol::tcp_window_scan6;
use pistol::tcp_xmas_scan;
use pistol::tcp_xmas_scan6;
use pistol::udp_scan;
use pistol::udp_scan6;
use pistol::Host;
use pistol::Host6;
use pistol::Target;
use std::net::IpAddr;
use std::time::Duration;
use std::time::Instant;

use crate::errors::InsufficientPortsError;
use crate::utils::addr_is_ipv6;
use crate::utils::target_addr_parser;
use crate::utils::target_port_parser;

pub enum ScanMethods {
    SYN,
    FIN,
    CON,
    ACK,
    NULL,
    XMAS,
    WIN,
    MAI,
    UDP,
}

pub fn portscan(
    addr: &str,
    port: &str,
    threads_num: usize,
    timeout: f32,
    method: ScanMethods,
) -> Result<()> {
    let addr_vec = target_addr_parser(addr)?;
    let port_vec = target_port_parser(port)?;
    let timeout = Duration::from_secs_f32(timeout);
    let start_time = Instant::now();

    let ret = if addr_is_ipv6(addr) {
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
        match method {
            ScanMethods::SYN => tcp_syn_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::FIN => tcp_fin_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::CON => tcp_connect_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::NULL => tcp_null_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::XMAS => tcp_xmas_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::ACK => tcp_ack_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::WIN => tcp_window_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::MAI => tcp_maimon_scan6(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::UDP => udp_scan6(target, None, None, threads_num, Some(timeout))?,
        }
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
        match method {
            ScanMethods::SYN => tcp_syn_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::FIN => tcp_fin_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::CON => tcp_connect_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::NULL => tcp_null_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::XMAS => tcp_xmas_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::ACK => tcp_ack_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::WIN => tcp_window_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::MAI => tcp_maimon_scan(target, None, None, threads_num, Some(timeout))?,
            ScanMethods::UDP => udp_scan(target, None, None, threads_num, Some(timeout))?,
        }
    };

    for (ip, s) in ret.results {
        for (port, status) in s.status {
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

    let dur = start_time.elapsed();
    println!("Time: {:.3}s", dur.as_secs_f32());

    Ok(())
}

struct OSScanPorts {
    tcp_open_port: u16,
    tcp_closed_port: u16,
    udp_closed_port: u16,
}

impl OSScanPorts {
    fn new(tcp_open_port: u16, tcp_closed_port: u16, udp_closed_port: u16) -> OSScanPorts {
        OSScanPorts {
            tcp_open_port,
            tcp_closed_port,
            udp_closed_port,
        }
    }
}

fn osscan_portscan(addr: IpAddr, threads_num: usize, timeout: Duration) -> Result<OSScanPorts> {
    // remote os detect port scan
    // return specific ports
    let mut tcp_open_port = 0;
    let mut tcp_closed_port = 0;
    let mut udp_closed_port = 0;

    let mut wellknown_ports: Vec<u16> = Vec::new();
    for p in 1..1024 {
        // not start with 0
        wellknown_ports.push(p);
    }
    let mut registered_ports: Vec<u16> = Vec::new();
    for p in 1024..49152 {
        registered_ports.push(p);
    }
    let mut private_ports: Vec<u16> = Vec::new();
    for p in 49152..=65535 {
        private_ports.push(p);
    }

    let all_ports = vec![wellknown_ports, registered_ports, private_ports];
    for ports in all_ports {
        // println!(
        //     "{} - {} - {}",
        //     tcp_open_port, tcp_closed_port, udp_closed_port
        // );
        if tcp_open_port != 0 && tcp_closed_port != 0 && udp_closed_port != 0 {
            break;
        }

        let target = match addr {
            IpAddr::V6(ipv6_addr) => {
                let host = Host6::new(ipv6_addr, Some(ports));
                let target = Target::new6(vec![host]);
                target
            }
            IpAddr::V4(ipv4_addr) => {
                let host = Host::new(ipv4_addr, Some(ports));
                let target = Target::new(vec![host]);
                target
            }
        };

        // println!("start tcp syn scan");
        if tcp_open_port == 0 || tcp_closed_port == 0 {
            let ret = match addr {
                IpAddr::V6(_ipv6_addr) => {
                    match tcp_syn_scan6(target.clone(), None, None, threads_num, Some(timeout)) {
                        Ok(ret) => Some(ret),
                        Err(_) => None, // ignore the error here
                    }
                }
                IpAddr::V4(_ipv4_addr) => {
                    match tcp_syn_scan(target.clone(), None, None, threads_num, Some(timeout)) {
                        Ok(ret) => Some(ret),
                        Err(_) => None, // ignore the error here
                    }
                }
            };
            match ret {
                Some(ret) => {
                    for (_ip, s) in ret.results {
                        for (port, status) in s.status {
                            match status {
                                TargetScanStatus::Open => {
                                    if tcp_open_port == 0 {
                                        tcp_open_port = port;
                                    }
                                }
                                _ => {
                                    if tcp_closed_port == 0 {
                                        tcp_closed_port = port;
                                    }
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }

        // println!("start udp scan");
        if udp_closed_port == 0 {
            let ret = match addr {
                IpAddr::V6(_ipv6_addr) => {
                    match udp_scan6(target, None, None, threads_num, Some(timeout)) {
                        Ok(ret) => Some(ret),
                        Err(_) => None, // ignore the error here
                    }
                }
                IpAddr::V4(_ipv4_addr) => {
                    match udp_scan(target, None, None, threads_num, Some(timeout)) {
                        Ok(ret) => Some(ret),
                        Err(_) => None, // ignore the error here
                    }
                }
            };

            match ret {
                Some(ret) => {
                    for (_ip, s) in ret.results {
                        for (port, status) in s.status {
                            match status {
                                TargetScanStatus::Open => (),
                                _ => {
                                    if udp_closed_port == 0 {
                                        udp_closed_port = port;
                                    }
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }
    }

    if tcp_open_port != 0 && tcp_closed_port != 0 && udp_closed_port != 0 {
        let ret = OSScanPorts::new(tcp_open_port, tcp_closed_port, udp_closed_port);
        Ok(ret)
    } else {
        let msg = format!("{}", addr);
        Err(InsufficientPortsError::new(&msg).into())
    }
}

pub fn osscan(addr: &str, threads_num: usize, timeout: f32, top_k: usize) -> Result<()> {
    let addr_vec = target_addr_parser(addr)?;
    let timeout = Duration::from_secs_f32(timeout);
    let start_time = Instant::now();

    if addr_is_ipv6(addr) {
        // ipv6
        let mut host_vec = Vec::new();
        for addr in addr_vec {
            let op = osscan_portscan(addr, threads_num, timeout)?;
            match addr {
                IpAddr::V6(addr) => {
                    let ports = vec![op.tcp_open_port, op.tcp_closed_port, op.udp_closed_port];
                    let host = Host6::new(addr, Some(ports));
                    host_vec.push(host);
                }
                _ => (),
            }
        }
        let target = Target::new6(host_vec);
        let ret = os_detect6(target, None, None, top_k, threads_num, Some(timeout))?;
        println!("{}", ret);
    } else {
        // ipv4
        let mut host_vec = Vec::new();
        for addr in addr_vec {
            let op = osscan_portscan(addr, threads_num, timeout)?;
            match addr {
                IpAddr::V4(addr) => {
                    let ports = vec![op.tcp_open_port, op.tcp_closed_port, op.udp_closed_port];
                    let host = Host::new(addr, Some(ports));
                    host_vec.push(host);
                }
                _ => (),
            }
        }
        let target = Target::new(host_vec);
        let ret = os_detect(target, None, None, top_k, threads_num, Some(timeout))?;
        println!("{}", ret);
    };

    let dur = start_time.elapsed();
    println!("Time: {:.3}s", dur.as_secs_f32());
    Ok(())
}
