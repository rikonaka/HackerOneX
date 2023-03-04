// use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::UdpSocket;

fn gen_port_vec(port_range: &str) -> (i32, i32) {
    let split = port_range.split("-");
    let vec: Vec<&str> = split.collect();
    if vec.len() == 2 {
        let start: i32 = vec[0].parse().unwrap();
        let end: i32 = vec[1].parse().unwrap();
        return (start, end);
    }
    (0, 0)
}

#[tokio::main]
pub async fn run(target: &str, port_range: &str, protocol: &str) {
    let (start, end) = gen_port_vec(port_range);
    println!("{} - {}", start, end);
    match protocol {
        "tcp" => {
            for port in start..end {
                // Connect to a peer
                let address = format!("{}:{}", target, port);
                let _ = match TcpStream::connect(address).await {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                println!("Port [{}] is open", port);
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
