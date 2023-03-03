use kdam::{tqdm, BarExt};
use reqwest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

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

fn get_url_from_file(path: &str) -> Vec<String> {
    fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
        // Open the file in read-only mode.
        let file = File::open(filename).unwrap();
        // Read the file line by line, and return an iterator of the lines of the file.
        return io::BufReader::new(file).lines();
    }
    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines(path.to_string());
    // Iterate over the lines of the file, and in this case print them.
    let mut result_vec = Vec::new();
    for line in lines {
        result_vec.push(line.unwrap());
    }
    result_vec
}

fn get_url_from_str(wordlists: &str) -> Vec<String> {
    let split = wordlists.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut result = Vec::new();
    for v in vec {
        result.push(v.to_string());
    }
    result
}

fn check_target(target: &str) -> String {
    let target = if target.chars().last().unwrap() == '/' {
        target.to_string()
    } else {
        let mut new_target = target.to_string();
        new_target.push_str("/");
        new_target
    };
    if target.contains("http") {
        target
    } else {
        let target = format!("http://{}", target);
        target
    }
}

// #[tokio::main(worker_threads = 32)]
#[tokio::main]
pub async fn run(path: &str, target: &str, wordlists: Option<&str>) -> Vec<String> {
    let start = Instant::now();
    let target = check_target(target);
    let url_vec = match wordlists {
        Some(w) => get_url_from_str(w),
        None => get_url_from_file(path),
    };
    let result_200 = match http_get(&target, url_vec).await {
        Ok(result) => result,
        Err(e) => panic!("Run http_get error: {}", e),
    };
    let duration = start.elapsed();
    println!("Exec time: {:?}", duration);
    result_200
}