use crate::Message;
use std::collections::HashMap;
use std::fs;

use std::convert::Infallible;
// use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

struct Maps {
    data: HashMap<String, String>,
}

impl Maps {
    fn new() -> Maps {
        let data = HashMap::new();
        Maps { data }
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn web(
    address: &str,
    port: u16,
    maps: &Maps,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = format!("{}:{}", address, port);

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn read_conf(file_path: &str) -> Maps {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);
    let lines = if cfg!(target_os = "linux") {
        contents.split("\n")
    } else if cfg!(target_os = "windows") {
        contents.split("\r\n")
    } else if cfg!(target_os = "macos") {
        contents.split("\r")
    } else {
        let e_str = "Unsupport OS!";
        e_str.to_string().error_message();
        panic!("{}", e_str);
    };
    let lines_vec: Vec<&str> = lines.collect();
    let mut maps = Maps::new();
    let mut request = String::new();
    let mut response = String::new();
    let mut read_request = false;
    let mut read_response = false;
    let mut need_save = false;
    for l in lines_vec {
        match l {
            "[request]" => {
                read_request = true;
                read_response = false;
                if response.len() > 0 {
                    maps.data.insert(request.clone(), response.clone());
                    request = String::new();
                    response = String::new();
                }
                need_save = false;
            }
            "[response]" => {
                read_request = false;
                read_response = true;
                need_save = true;
            }
            _ => {
                if read_request {
                    request.push_str(l);
                    maps.data.insert(l.to_string(), "".to_string());
                } else if read_response {
                    response.push_str(l);
                    response.push_str("\r\n");
                }
            }
        }
    }
    if need_save {
        if response.len() & request.len() > 0 {
            // println!("{}, {}", request, response);
            maps.data.insert(request.clone(), response.clone());
        }
    }
    // println!("{:?}", maps.data);
    maps
}

pub fn run(address: &str, port: u16) {
    let path = "./src/honeypot/response.txt";
    let maps = read_conf(path);
    match web(address, port, &maps) {
        Ok(_) => (),
        Err(e) => {
            let e_str = format!("Running honeypot web error: {}", e);
            e_str.error_message();
        }
    }
}
