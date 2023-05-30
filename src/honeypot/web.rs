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
use once_cell::sync::OnceCell;
use tokio::net::TcpListener;

static URI_MAPS: OnceCell<HashMap<String, String>> = OnceCell::new();

fn read_conf(file_path: &str) -> HashMap<String, String> {
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
    let mut maps = HashMap::new();
    let mut request = String::new();
    let mut response = String::new();
    let mut read_default = false;
    let mut read_request = false;
    let mut read_response = false;
    let mut need_save = false;
    for l in lines_vec {
        let lt = l.trim();
        if lt.len() > 0 {
            match lt {
                "[default]" => {
                    read_default = true;
                }
                "[request]" => {
                    read_request = true;
                    read_response = false;
                    if response.len() > 0 {
                        maps.insert(request.clone(), response.clone());
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
                    if read_default {
                        maps.insert("default".to_string(), l.to_string());
                        read_default = false;
                    } else if read_request {
                        request.push_str(l);
                        maps.insert(l.to_string(), "".to_string());
                    } else if read_response {
                        response.push_str(l);
                        response.push_str("\r\n");
                    }
                }
            }
        }
    }
    if need_save {
        if response.len() & request.len() > 0 {
            // println!("{}, {}", request, response);
            maps.insert(request.clone(), response.clone());
        }
    }
    // println!("{:?}", maps.data);
    maps
}

async fn process_connection(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // println!("{:?}", req.uri());
    // let maps = read_conf(&config_path);
    let uri = req.uri().to_string();
    let maps = URI_MAPS.get().unwrap();
    let response = match maps.get(&uri) {
        Some(response) => response.as_str(),
        _ => maps.get("default").unwrap().as_str(),
    };
    Ok(Response::new(Full::new(Bytes::from(response))))
}

#[tokio::main]
async fn web(
    address: &str,
    port: u16,
    config: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // test service
    let maps = read_conf(config);
    URI_MAPS.set(maps).unwrap();

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = format!("{}:{}", address, port);
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(&addr).await?;
    format!("Running honeypot at {}..", &addr).info_message();
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            match http1::Builder::new()
                .serve_connection(
                    stream,
                    // service_fn(move |req| process_connection(req, inner)),
                    service_fn(process_connection),
                )
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    let e_str = format!("Error serving connection: {:?}", e);
                    e_str.error_message();
                }
            }
        });
    }
}

pub fn run(address: &str, port: u16, config: &str) {
    // let config = "./src/honeypot/response.txt";
    // fs::write(HIDEN_CONFIG, &config).unwrap();
    match web(address, port, config) {
        Ok(_) => (),
        Err(e) => {
            let e_str = format!("Running honeypot web error: {}", e);
            e_str.error_message();
        }
    }
}
