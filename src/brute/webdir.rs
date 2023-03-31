use crate::Message;
use reqwest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

async fn http_get(
    target: &str,
    url_vec: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut success_result: Vec<String> = Vec::new();
    for url in url_vec {
        if url.len() > 0 {
            let new_url = format!("{}{}", target, url);
            println!("Scan {}", &new_url);
            let res = client.get(&new_url).send().await?;
            match res.status() {
                reqwest::StatusCode::OK => {
                    println!("URL: {} - 200", &new_url.trim());
                    success_result.push(new_url);
                }
                _ => (),
            };
        }
        // pb.update(1);
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
    // de_duplication(result_vec)
    result_vec
}

fn get_url_from_str(wordlists: &str) -> Vec<String> {
    let split = wordlists.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut result = Vec::new();
    for v in vec {
        result.push(v.to_string());
    }
    // de_duplication(result)
    result
}

fn check_target(target: &str) -> String {
    let target = target.trim();
    let last_char = match target.chars().last() {
        Some(l) => l,
        None => panic!("Target last char not found: {}", target),
    };
    let target = if last_char == '/' {
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
pub async fn run(path: &str, target: &str, wordlists: Option<&str>) {
    let start = Instant::now();
    let mut target_vec = vec![check_target(target)];
    let url_vec = match wordlists {
        Some(w) => get_url_from_str(w),
        None => get_url_from_file(path),
    };
    let mut found_result = Vec::new();
    let mut index = 0;
    loop {
        if index >= target_vec.len() {
            break;
        }
        let target = target_vec.get(index).unwrap();
        let result_200 = match http_get(&target, &url_vec).await {
            Ok(result) => result,
            Err(e) => panic!("Run http_get error: {}", e),
        };
        // println!("{}", result_200.len());
        if result_200.len() != 0 {
            for r in &result_200 {
                found_result.push(r.clone());
            }
        }
        target_vec.extend(result_200);
        index += 1;
    }
    let duration = start.elapsed();
    println!("Exec time: {:?}", duration);

    for f in found_result {
        let info = format!("Found {}", f);
        info.info_message();
    }
}
