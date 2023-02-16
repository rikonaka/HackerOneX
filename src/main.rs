use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::vec::Vec;

/// Simple program to search vulnerability fast
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the vulnerability
    #[arg(short, long, default_value = "discuz")]
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ExploitalertResponse {
    id: String,
    date: String,
    name: String,
}

async fn exploitalert(
    exp_name: &str,
) -> Result<Vec<ExploitalertResponse>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.exploitalert.com/api/search-exploit?name={}",
        exp_name
    );

    let client = Client::new();
    let resp = client.get(url).send().await?.text().await?;
    // println!("{:#?}", resp);
    let serde_result: Vec<ExploitalertResponse> = serde_json::from_str(&resp)?;
    // println!("{:?}", serde_result.len());
    Ok(serde_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_exploitalert() {
        match exploitalert("discuz").await {
            Ok(_) => (),
            Err(e) => panic!("exploitalert error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let start_time = SystemTime::now();
    let search_result = match exploitalert(&args.name).await {
        Ok(result) => result,
        Err(e) => panic!("exploitalert error: {}", e),
    };
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    println!(
        "NOW LIST THE ALL RESULT ({:.3}s) (Length: {}):",
        duration.as_secs_f32(),
        search_result.len()
    );
    let mut index_title = " INDEX ".to_string();
    let mut cve_title = " CVE ".to_string();
    let mut date_title = " DATE ".to_string();
    let mut name_title = " NAME ".to_string();
    let mut index_max_len = index_title.len();
    let mut cve_max_len = cve_title.len();
    let mut date_max_len = date_title.len();
    let mut name_max_len = name_title.len();
    for (i, v) in search_result.iter().enumerate() {
        let index_str = format!(" {} ", i);
        if index_str.len() > index_max_len {
            index_max_len = index_str.len();
        }
        if v.id.len() + 2 > cve_max_len {
            cve_max_len = v.id.len() + 2;
        }
        if v.date.len() + 2 > date_max_len {
            date_max_len = v.date.len() + 2;
        }
        if v.name.len() + 2 > name_max_len {
            name_max_len = v.name.len() + 2;
        }
    }
    let balance_string = |title: String, title_max_len: usize| -> String {
        let mut new_title = title.clone();
        if title_max_len > title.len() {
            for i in 0..title_max_len - title.len() {
                if i % 2 == 0 {
                    new_title = format!(" {}", new_title);
                } else {
                    new_title = format!("{} ", new_title);
                }
            }
        }
        new_title
    };
    let balance_string_left = |title: String, title_max_len: usize| -> String {
        let mut new_title = title.clone();
        if title_max_len > title.len() {
            for _ in 0..title_max_len - title.len() {
                new_title = format!(" {}", new_title);
            }
        }
        new_title
    };
    let balance_string_right = |title: String, title_max_len: usize| -> String {
        let mut new_title = title.clone();
        if title_max_len > title.len() {
            for _ in 0..title_max_len - title.len() {
                new_title = format!("{} ", new_title);
            }
        }
        new_title
    };
    index_title = balance_string(index_title, index_max_len);
    cve_title = balance_string(cve_title, cve_max_len);
    date_title = balance_string(date_title, date_max_len);
    name_title = balance_string_right(name_title, name_max_len);
    println!(
        "|{}|{}|{}|{}|",
        index_title, cve_title, date_title, name_title
    );
    for (i, res) in search_result.iter().enumerate() {
        let i_str = format!(" {} ", i);
        let i_str = balance_string(i_str, index_max_len);
        let cve_str = format!(" {} ", res.id);
        let cve_str = balance_string_left(cve_str, cve_max_len);
        let date_str = format!(" {} ", res.date);
        let date_str = balance_string(date_str, date_max_len);
        let name_str = format!(" {} ", res.name);
        let name_str = balance_string_right(name_str, name_max_len);
        println!("|{}|{}|{}|{}|", i_str, cve_str, date_str, name_str);
    }
}
