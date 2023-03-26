// use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::UdpSocket;
use kdam::{tqdm, BarExt};

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
    println!("{} - {}", start, end);
    if end > start {
        match protocol {
            "tcp" => {
                let mut pb = tqdm!(total = end - start);
                for port in start..end {
                    // Connect to a peer
                    let address = format!("{}:{}", target, port);
                    match TcpStream::connect(address).await {
                        Ok(_) => {
                            println!("Port [{}] is open", port);
                        },
                        Err(_) => (),
                    };
                    pb.update(1);
                }
            }
            _ => {
                let sock = match UdpSocket::bind("0.0.0.0:19876").await {
                    Ok(s) => s,
                    _ => panic!("UDP listen at port 19876 failed"),
                };
                let mut buf = [0; 1024];
                let data = "scan".as_bytes();
                for port in start..end {
                    // Connect to a peer
                    let address = format!("{}:{}", target, port);
                    sock.connect(address).await.unwrap();
                    sock.send(data).await.unwrap();
                    match sock.recv(&mut buf).await {
                        Ok(len) => {
                            if len == 0 {
                                continue;
                            }
                        }
                        Err(_) => continue,
                    }
                    println!("Port [{}] is open", port);
                }
            }
        }
    }
}
