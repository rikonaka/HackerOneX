use kdam::{tqdm, BarExt};
use tokio::net::TcpStream;
// use tokio::net::UdpSocket;

use crate::Message;

fn gen_port_vec(port_range: &str) -> (usize, usize) {
    let split = port_range.split("-");
    let vec: Vec<&str> = split.collect();
    if vec.len() == 2 {
        let start = vec[0].parse().unwrap();
        let end = vec[1].parse().unwrap();
        return (start, end);
    }
    (0, 0)
}

#[tokio::main]
pub async fn run(target: &str, port_range: &str, protocol: &str) {
    let (start, end) = gen_port_vec(port_range);
    let message = format!("start port: {}, end port: {}", start, end);
    message.info_message();
    if end > start {
        let mut pb = tqdm!(total = end - start);
        match protocol {
            "udp" | "UDP" => {
                /*
                let sock = match UdpSocket::bind("0.0.0.0:19876").await {
                    Ok(s) => s,
                    _ => panic!("UDP listen at port 19876 failed"),
                };
                let data = "scan".as_bytes();
                for port in start..end {
                    // Connect to a peer
                    let address = format!("{}:{}", target, port);
                    sock.connect(&address).await.unwrap();
                    let message = format!("Connect to UDP: {}", &address);
                    message.verbose_message();
                    match sock.send(data).await {
                        Ok(ns) => {
                            if ns > 0 {
                                let message = format!("Send UDP packet to [{}] success", &address);
                                message.verbose_message();
                                let message = format!("Port [{}] is open", port);
                                // message.info_message();
                                let info_message = message.get_info_message();
                                pb.write(info_message);
                            }
                        }
                        Err(e) => {
                            let message =
                                format!("Send UDP packets to [{}] failed: {}", &address, e);
                            message.verbose_message();
                        }
                    }
                    pb.update(1);
                }
                */
                let message = "UDP scan is not finish..";
                message.to_string().warning_message();
            }
            "tcp" | "TCP" => {
                // very simple TCP CONNECT SCAN
                for port in start..end {
                    // Connect to a peer
                    let address = format!("{}:{}", target, port);
                    match TcpStream::connect(address).await {
                        Ok(_) => {
                            let message = format!("Port [{}] is open", port);
                            // message.info_message();
                            let info_message = message.get_info_message();
                            pb.write(info_message);
                        }
                        Err(_) => (),
                    };
                    pb.update(1);
                }
            }
            _ => {
                let message = format!("Unknown protocol: {}", protocol);
                message.error_message();
            }
        }
    } else {
        let message = format!(
            "The value of end [{}] should bigger than start [{}]",
            end, start
        );
        message.error_message();
    }
}

#[tokio::test]
async fn udp_test() -> std::io::Result<()> {
    // Bind socket
    use tokio::net::UdpSocket;
    let socket = UdpSocket::bind("127.0.0.1:28080").await?;
    socket.connect("127.0.0.1:2115").await?;

    // Send a message
    socket.send(b"hello world").await?;

    Ok(())
}
