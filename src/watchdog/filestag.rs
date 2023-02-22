use md5;
use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    id: usize,
    path: String,
    hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilesInfo {
    list: Vec<FileInfo>,
}

pub fn run(target_path: &str) {
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
    let filesinfo = FilesInfo {
        list: fileinfo_list,
    };

    // println!("{}", serde_json::to_string(&filesinfo).unwrap());
    let mut file = match Path::new("filestag.db").exists() {
        true => File::open("foo.txt").unwrap(),
        _ => File::create("foo.txt").unwrap(),
    };
    file.write_all(&serde_json::to_vec(&filesinfo).unwrap())
        .unwrap();
}
