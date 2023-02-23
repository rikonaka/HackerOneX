use ctrlc;
use md5;
use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;
use std::{thread, time};
use walkdir::WalkDir;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FileInfo {
    id: usize,
    path: String,
    hash: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FilesInfo {
    list: Vec<FileInfo>,
}

const DB_FILE: &str = "filestag.db";

fn walk_files(target_path: &str) -> Vec<FileInfo> {
    let mut path_list: Vec<String> = Vec::new();
    for entry in WalkDir::new(target_path).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        path_list.push(entry.path().display().to_string());
    }
    // println!("{:?}", files);
    let mut fileinfo_list = Vec::new();
    for (i, path) in path_list.iter().enumerate() {
        // println!("{}", path);
        let md = metadata(&path).unwrap();
        match md.is_file() {
            true => {
                let f = File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).unwrap();
                let digest = md5::compute(buffer);
                // println!("{:?}", digest);
                // break;
                let fi = FileInfo {
                    id: i,
                    path: path.to_string(),
                    hash: digest.to_vec(),
                };
                fileinfo_list.push(fi)
            }
            _ => (),
        }
    }
    fileinfo_list
}

fn save_db(filesinfo: FilesInfo) {
    let mut file = match Path::new(DB_FILE).exists() {
        true => File::open(DB_FILE).unwrap(),
        _ => File::create(DB_FILE).unwrap(),
    };
    // println!("{}", serde_json::to_string(&filesinfo).unwrap());
    file.write_all(&serde_json::to_vec(&filesinfo).unwrap())
        .unwrap();
}

fn load_db() -> Option<FilesInfo> {
    match Path::new(DB_FILE).exists() {
        true => {
            let f = File::open(DB_FILE).unwrap();
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();

            let filesinfo: FilesInfo = serde_json::from_slice(&buffer).unwrap();
            // println!("{:?}", new_filesinfo);
            Some(filesinfo)
        }
        _ => None,
    }
}

pub fn run(target_path: &str, delay: f32) {
    let delay_duration = time::Duration::from_secs_f32(delay);
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let mut saved_filesinfo = load_db();
    loop {
        let fileinfo_list = walk_files(target_path);
        match saved_filesinfo {
            Some(fsi) => {
                let saved_fileinfo_list = fsi.list;
            },
            _ => {
                saved_filesinfo = Some(FilesInfo{list: fileinfo_list.clone()});
            }
        }
        match rx.recv() {
            Ok(_) => {
                let filesinfo = FilesInfo {
                    list: fileinfo_list,
                };
                save_db(filesinfo);
                println!("stop filestag...");
            }
            Err(_) => (),
        }
        thread::sleep(delay_duration);
    }
}
