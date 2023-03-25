use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use sqlite;
use sqlite::{Connection, State};
use std::{fs, thread, time};
use walkdir::WalkDir;

use crate::Message;
use crate::NULL;
use crate::VERBOSE_FLAG;

const DBFILE: &str = "filestag.db";

struct Database {
    connection: Connection,
}

impl Database {
    fn connect() -> Self {
        let connection = sqlite::open(DBFILE).unwrap();
        let sql_str = "CREATE TABLE IF NOT EXISTS files (path TEXT UNIQUE, md5 TEXT, sha1 TEXT, last_opt TEXT);";
        connection.execute(sql_str).unwrap();
        Database { connection }
    }
    fn insert(&self, path: &String, md5: &String, sha1: &String, last_opt: &String) {
        // println!("{}", file_path);
        // println!("sha1 = {}", sha1);
        // println!("md5 = {}", md5);
        let sql_str = format!(
            "INSERT INTO files (path, md5, sha1, last_opt) VALUES ('{}', '{}', '{}', '{}');",
            path, md5, sha1, last_opt
        );
        self.connection.execute(sql_str).unwrap();
    }
    fn update(&self, path: &String, md5: &String, sha1: &String, last_opt: &String) {
        let sql_str = format!(
            "UPDATE files SET md5='{}', sha1='{}', last_opt='{}' WHERE path='{}'",
            md5, sha1, last_opt, path
        );
        self.connection.execute(sql_str).unwrap();
    }
    fn select(&self, path: &String) -> Vec<Vec<String>> {
        // let sql_str = format!("SELECT * FROM files;");
        let sql_str = format!("SELECT * FROM files WHERE path='{}';", path);
        let mut statement = self.connection.prepare(sql_str).unwrap();
        let mut result_v: Vec<Vec<String>> = Vec::new();

        while let State::Row = statement.next().unwrap() {
            let path = statement.read::<String, _>("path").unwrap();
            let md5 = statement.read::<String, _>("md5").unwrap();
            let sha1 = statement.read::<String, _>("sha1").unwrap();
            let last_opt = statement.read::<String, _>("last_opt").unwrap();
            let tmp_v: Vec<String> = vec![path, md5, sha1, last_opt];
            result_v.push(tmp_v);
        }
        result_v
    }
    fn select_all(&self) -> Vec<Vec<String>> {
        // let sql_str = format!("SELECT * FROM files;");
        let sql_str = format!("SELECT * FROM files;");
        let mut statement = self.connection.prepare(sql_str).unwrap();
        let mut result_v: Vec<Vec<String>> = Vec::new();

        while let State::Row = statement.next().unwrap() {
            let path = statement.read::<String, _>("path").unwrap();
            let md5 = statement.read::<String, _>("md5").unwrap();
            let sha1 = statement.read::<String, _>("sha1").unwrap();
            let last_opt = statement.read::<String, _>("last_opt").unwrap();
            let tmp_v: Vec<String> = vec![path, md5, sha1, last_opt];
            result_v.push(tmp_v);
        }
        result_v
    }
    fn show_all(&self) {
        let sql_str = format!("SELECT * FROM files;");
        let mut statement = self.connection.prepare(sql_str).unwrap();
        while let State::Row = statement.next().unwrap() {
            let path = format!("path = {}", statement.read::<String, _>("path").unwrap());
            let md5 = format!("md5 = {}", statement.read::<String, _>("md5").unwrap());
            let sha1 = format!("sha1 = {}", statement.read::<String, _>("sha1").unwrap());
            let last_opt = format!(
                "last_opt = {}",
                statement.read::<String, _>("last_opt").unwrap()
            );
            path.verbose_message();
            md5.verbose_message();
            sha1.verbose_message();
            last_opt.verbose_message();
        }
    }
}

fn md5_cal(data: &Vec<u8>) -> Option<String> {
    let mut md5_hasher = Md5::new();
    md5_hasher.input(data);
    Some(md5_hasher.result_str())
}

fn sha1_cal(data: &Vec<u8>) -> Option<String> {
    let mut sha1_hasher = Sha1::new();
    sha1_hasher.input(data);
    Some(sha1_hasher.result_str())
}

fn all_files(target_dir: &String) -> Vec<String> {
    let mut result_v: Vec<String> = Vec::new();
    if target_dir.len() > 0 {
        for entry in WalkDir::new(target_dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                // println!("{}", path.display());
                result_v.push(path.to_string_lossy().to_string())
            }
        }
    }
    result_v
}

fn forward(db: &Database, watchpath: &str) {
    /* Travel all files to find file change or file add
     */
    let all_files_v = all_files(&watchpath.to_string());
    for f in &all_files_v {
        let data = fs::read(f).expect("Something went wrong reading the file");
        let md5 = md5_cal(&data).unwrap();
        let sha1 = sha1_cal(&data).unwrap();
        let query_result_v = db.select(&f);
        if query_result_v.len() > 0 {
            for q in query_result_v {
                let md5_old = &q[1];
                let sha1_old = &q[2];
                if (*md5_old != md5) && (*sha1_old != sha1) {
                    let log_str = format!("Some file changed: {}", f);
                    log_str.info_message();
                    // log_to_file(&log_str);
                    // println!("update");
                    let last_opt = String::from("changed");
                    db.update(&f, &md5, &sha1, &last_opt);
                }
            }
        } else {
            // println!("insert");
            let log_str = format!("Some file added: {}", f);
            log_str.info_message();
            // log_to_file(&log_str);
            let last_opt = String::from("Added");
            db.insert(&f, &md5, &sha1, &last_opt);
        }
    }
}

fn reverse(db: &Database, watchpath: &str) {
    /* Travel all database stored files to find file delete
     */
    let db_files_v = db.select_all();
    let all_files_v = all_files(&watchpath.to_string());
    for df in db_files_v {
        let last_opt = &df[3];
        let path = &df[0];
        if !all_files_v.contains(path) && last_opt != "deleted" {
            let log_str = format!("Some file deleted: {}", path);
            let null = String::from(NULL);
            let last_opt = String::from("deleted");
            db.update(path, &null, &null, &last_opt);
            // log_to_file(&log_str);
            log_str.info_message();
        }
    }
}

pub fn run(path: &str, delay: f32) {
    let db = Database::connect();
    let verbose = VERBOSE_FLAG.get().unwrap();
    match verbose {
        true => db.show_all(),
        _ => (),
    }
    "diskwatch runing...".to_string().info_message();
    loop {
        forward(&db, path);
        reverse(&db, path);
        // sleep 1s
        let ten_millis = time::Duration::from_secs_f32(delay);
        thread::sleep(ten_millis);
    }
}
