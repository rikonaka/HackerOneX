use kdam::{tqdm, BarExt};
use reqwest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::{Duration, Instant};

// #[tokio::main(worker_threads = 32)]
#[tokio::main(worker_threads = 32)]
async fn http_get(
    target: &str,
    url_vec: Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut success_result: Vec<String> = Vec::new();
    let mut pb = tqdm!(total = url_vec.len());
    for url in url_vec {
        let new_url = format!("{}{}", target, url);
        // println!("{}", new_url);
        let res = client.get(&new_url).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let message = format!("URL: {} - 200", &new_url);
                pb.write(message);
                success_result.push(new_url);
            }
            _ => {
                // other => {
                // panic!("Uh oh! Something unexpected happened: {:?}", other);
                // println!("URL: {} - {}", &new_url, other.as_u16());
            }
        };
        pb.update(1);
    }
    Ok(success_result)
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}
fn get_url(path: &str) -> Vec<String> {
    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines(path.to_string());
    // Iterate over the lines of the file, and in this case print them.
    let mut result_vec = Vec::new();
    for line in lines {
        result_vec.push(line.unwrap());
    }
    result_vec
}

fn check_target(target: &str) -> String {
    if target.chars().last().unwrap() == '/' {
        target.to_string()
    } else {
        let mut new_target = target.to_string();
        new_target.push_str("/");
        new_target
    }
}

pub fn run(path: &str, target: &str) -> Vec<String> {
    let start = Instant::now();
    let target = check_target(target);
    let url_vec = get_url(path);
    let result_200 = match http_get(&target, url_vec) {
        Ok(result) => result,
        Err(e) => panic!("Run http_get error: {}", e),
    };
    let duration = start.elapsed();
    println!("Exec time is: {:?}", duration);
    result_200
}
