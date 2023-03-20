use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::Message;

pub struct TinyDatabase {
    database: HashMap<String, String>,
}

impl TinyDatabase {
    fn new() -> TinyDatabase {
        TinyDatabase {
            database: HashMap::new(),
        }
    }
    fn get(&self, key: &str) -> Option<String> {
        match self.database.get(key) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
    fn set(&mut self, key: &str, value: &str) {
        self.database.insert(key.to_string(), value.to_string());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackendCommand {
    command: String,
    key: String,
    value: String,
}

impl BackendCommand {
    pub fn new(
        command: Option<String>,
        key: Option<String>,
        value: Option<String>,
    ) -> BackendCommand {
        let default_null_value = String::from("null");
        let name_ = match command {
            Some(n) => n,
            _ => default_null_value.clone(),
        };
        let key_ = match key {
            Some(k) => k,
            _ => default_null_value.clone(),
        };
        let value_ = match value {
            Some(v) => v,
            _ => default_null_value.clone(),
        };
        BackendCommand {
            command: name_,
            key: key_,
            value: value_,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    pub fn deserialize(data: &[u8]) -> BackendCommand {
        serde_json::from_slice(&data).unwrap()
    }
    pub async fn connect_backend(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Open a TCP stream to the socket address.
        let addr = format!("127.0.0.1:{}", DEFAULT_PORT);
        // Note that this is the Tokio TcpStream, which is fully async.
        let mut stream = TcpStream::connect(&addr).await?;
        // println!("created stream");

        match stream.write(&Self::serialize(&self)).await {
            Ok(_) => {
                // println!("Send data to backend success: {}", res);
            }
            Err(e) => {
                let e_str = format!("Failed to write to backend service: {}", e);
                e_str.error_message();
            }
        }
        // println!("wrote to stream; success={:?}", result.is_ok());
        let mut buf = vec![0; BUFF_SIZE];
        let n = match stream.read(&mut buf).await {
            Ok(n) => n,
            Err(e) => {
                let e_str = format!("Failed to read from backend service: {}", e);
                e_str.error_message();
                0
            }
        };
        // println!("client n: {}", n);
        // println!("{:?}", buf);
        Ok(buf[0..n].to_vec())
    }
}

const DEFAULT_PORT: u16 = 23333;
const BUFF_SIZE: usize = 4096 * 100 * 100;

#[tokio::main]
async fn tcp_server() -> Result<(), Box<dyn Error>> {
    let mut tb = TinyDatabase::new();
    let addr = format!("127.0.0.1:{}", DEFAULT_PORT);

    let listener = TcpListener::bind(&addr).await?;
    println!("Backend listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        // tokio::spawn(async move {
        let mut buf = vec![0; BUFF_SIZE];
        loop {
            let n = match socket.read(&mut buf).await {
                Ok(n) => n,
                Err(_) => break,
            };
            if n > 0 {
                let bc = BackendCommand::deserialize(&buf[0..n]);
                let response = match bc.command.as_str() {
                    "set" => {
                        tb.set(&bc.key, &bc.value);
                        "Ok".to_string()
                    }
                    "get" => {
                        let value = match tb.get(&bc.key) {
                            Some(v) => v,
                            _ => "null".to_string(),
                        };
                        value
                    }
                    _ => "Unknown command".to_string(),
                };
                match socket.write_all(response.as_bytes()).await {
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
            // println!("server n: {}", n);
            else if n == 0 {
                break;
            }
        }
        // });
    }
}

pub fn run() {
    match tcp_server() {
        Ok(_) => (),
        Err(e) => {
            let e_str = format!("Running backend service error: {}", e);
            e_str.error_message();
        }
    }
}
