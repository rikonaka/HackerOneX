use crate::Message;
use kdam::{tqdm, BarExt};
use reqwest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

async fn http_get(
    target: &str,
    url_vec: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let max_line_len = 30;
    let client = reqwest::Client::new();
    let mut success_result: Vec<String> = Vec::new();
    let mut pb = tqdm!(total = url_vec.len());
    for url in url_vec {
        if url.len() > 0 {
            let mut new_url = format!("{}{}", target, url);
            let res = client.get(&new_url).send().await?;
            if new_url.len() >= max_line_len {
                pb.set_description(format!("SCAN {}", &new_url[..max_line_len]));
            } else {
                for _ in 0..(max_line_len - new_url.len()) {
                    new_url = format!("{} ", new_url);
                }
                pb.set_description(format!("SCAN {}", &new_url));
            }
            // println!("{}", new_url);
            match res.status() {
                reqwest::StatusCode::OK => {
                    let message = format!("URL: {} - 200", &new_url);
                    pb.write(message);
                    // println!("{}", message);
                    success_result.push(new_url);
                }
                _ => {
                    // other => {
                    // let space_str = " ".repeat(100);
                    // print!("\rURL: {} - {:?}{}", &new_url, other, space_str);
                    // let message = format!("URL: {} - {:?}", &new_url, other);
                    // pb.write(message);
                }
            };
        }
        pb.update(1);
    }
    Ok(success_result)
}

/*
fn de_duplication(input: Vec<String>) -> Vec<String> {
    "de-duplication...".to_string().info();
    let mut result = Vec::new();
    for i in input {
        if !result.contains(&i) {
            result.push(i);
        }
    }
    result
}
*/

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

fn show_result(input: &Vec<String>) {
    for i in input {
        i.info_message();
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
    let mut index = 0;
    loop {
        if index >= target_vec.len() {
            break;
        }
        let target = target_vec.get(index).unwrap();
        index += 1;
        let result_200 = match http_get(&target, &url_vec).await {
            Ok(result) => result,
            Err(e) => panic!("Run http_get error: {}", e),
        };
        println!("{}", result_200.len());
        if result_200.len() == 0 {
            break;
        }
        show_result(&result_200);
        target_vec.extend(result_200);
    }
    let duration = start.elapsed();
    println!("Exec time: {:?}", duration);
}
