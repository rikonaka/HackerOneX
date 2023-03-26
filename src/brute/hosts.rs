use kdam::{tqdm, BarExt};
use surge_ping::IcmpPacket;

use crate::Message;

#[tokio::main]
async fn ping(ipvec: Vec<String>) {
    let mut pb = tqdm!(total = ipvec.len());
    for ip in ipvec {
        match surge_ping::ping(ip.parse().unwrap(), &[0]).await {
            Ok((IcmpPacket::V4(packet), duration)) => {
                // println!(
                //     "{} bytes from {}: icmp_seq={} ttl={:?} time={:.2?}",
                //     packet.get_size(),
                //     packet.get_source(),
                //     packet.get_sequence(),
                //     packet.get_ttl(),
                //     duration
                // );
                let message = format!(
                    "host is alive: {}, ttl: {}, time: {:.2?}",
                    ip,
                    packet.get_ttl(),
                    duration
                );
                let info_message = message.get_info_message();
                pb.write(info_message);
            }
            Ok(_) => {
                // unreachable!();
            }
            Err(e) => println!("{:?}", e),
        };
        pb.update(1);
    }
}

fn subnet_error(subnet: &str) {
    let message = format!("subnet is wrong: {}", subnet);
    message.error_message();
}

pub fn run(subnet: &str) {
    let subnet_split = subnet.split(".");
    let subnet_vec: Vec<&str> = subnet_split.collect();
    if subnet_vec.len() < 4 {
        subnet_error(subnet);
    } else {
        let last_number: usize = subnet_vec[3].parse().unwrap();
        let mut ipvec: Vec<String> = Vec::new();
        if last_number + 1 < 255 {
            for i in (last_number + 1)..255 {
                let ip = format!(
                    "{}.{}.{}.{}",
                    subnet_vec[0], subnet_vec[1], subnet_vec[2], i
                );
                ipvec.push(ip);
            }
            println!("{:?}", ipvec);
            ping(ipvec);
        } else {
            subnet_error(subnet);
        }
    }
}
