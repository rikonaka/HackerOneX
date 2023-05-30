use kdam::{tqdm, BarExt};

use crate::Message;

#[tokio::main]
async fn ping(ipvec: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let payload = [0; 8];
    let mut pb = tqdm!(total = ipvec.len());
    for ip in ipvec {
        let (_packet, duration) = surge_ping::ping(ip.parse().unwrap(), &payload).await?;
        let message = format!(
            "host is alive: {}, seq: {}, time: {:.2?}",
            ip,
            _packet.get_sequence(),
            duration
        );
        let info_message = message.get_info_message();
        pb.write(info_message);
        pb.update(1);
    }
    Ok(())
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
            // #println!("{:?}", ipvec);
            ping(ipvec).unwrap();
        } else {
            subnet_error(subnet);
        }
    }
}
