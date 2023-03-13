use crate::Message;
use chrono;
use sqlx::Column;
use sqlx::Connection;
use sqlx::MySqlConnection;
use sqlx::PgConnection;
use sqlx::Row;
use sqlx::TypeInfo;
use std::collections::HashMap;

struct SqlData {
    string_type: HashMap<String, String>,
    int_type: HashMap<String, i32>,
    float_type: HashMap<String, f32>,
    date_type: HashMap<String, chrono::DateTime<chrono::Utc>>,
    unsupport_type: HashMap<String, Vec<u8>>,
}

impl SqlData {
    fn new() -> SqlData {
        let string_type = HashMap::new();
        let int_type = HashMap::new();
        let float_type = HashMap::new();
        let date_type = HashMap::new();
        let unsupport_type = HashMap::new();
        SqlData {
            string_type,
            int_type,
            float_type,
            date_type,
            unsupport_type,
        }
    }
    fn get(&self, col_name: &str) -> String {
        let value = if self.string_type.contains_key(col_name) {
            match self.string_type.get(col_name) {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            }
        } else if self.int_type.contains_key(col_name) {
            match self.int_type.get(col_name) {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            }
        } else if self.float_type.contains_key(col_name) {
            match self.float_type.get(col_name) {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            }
        } else if self.date_type.contains_key(col_name) {
            match self.date_type.get(col_name) {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            }
        } else if self.unsupport_type.contains_key(col_name) {
            match self.unsupport_type.get(col_name) {
                Some(v) => {
                    // String::from_utf8_lossy(v).to_string()
                    match String::from_utf8(v.clone()) {
                        Ok(s) => s.to_string(),
                        Err(_) => "<binary>".to_string(),
                    }
                }
                None => "null".to_string(),
            }
        } else {
            "null".to_string()
        };
        value
    }
}

struct SqlDatas {
    col_name_vec: Vec<String>,
    max_col_len: HashMap<String, usize>,
    data: Vec<SqlData>,
}

impl SqlDatas {
    fn new() -> SqlDatas {
        let max_col_len = HashMap::new();
        let col_name_vec = Vec::new();
        let data: Vec<SqlData> = Vec::new();
        SqlDatas {
            col_name_vec,
            max_col_len,
            data,
        }
    }
    fn push_sql_data(&mut self, sql_data: SqlData) {
        self.data.push(sql_data);
    }
    fn push_col_name(&mut self, col_name: &str) {
        if !self.col_name_vec.contains(&col_name.to_string()) {
            self.col_name_vec.push(col_name.to_string());
        }
    }
    fn cal_max_col_len(&mut self) {
        for col_name in &self.col_name_vec {
            self.max_col_len
                .insert(col_name.to_string(), col_name.len() + 2);
        }
        // calculate the max col len
        for d in &self.data {
            for col_name in &self.col_name_vec {
                let value = d.get(&col_name);
                if value.len() + 2 > *self.max_col_len.get(col_name).unwrap() {
                    self.max_col_len
                        .insert(col_name.to_string(), value.len() + 2);
                }
            }
        }
    }
    fn show(&mut self) {
        if self.col_name_vec.len() > 0 {
            Self::cal_max_col_len(self);
            let mut col_string = String::from("|");
            let mut hline_string = String::from("+");
            for col_name in &self.col_name_vec {
                let need_pad_len = (self.max_col_len[col_name] - col_name.len()) as i32;
                let mut col_name = col_name.to_string();
                for i in 0..need_pad_len {
                    if i % 2 == 0 {
                        col_name = format!("{} ", col_name);
                    } else {
                        col_name = format!(" {}", col_name);
                    }
                }
                col_string = format!("{}{}|", col_string, col_name);
                let mut hline = String::new();
                for _ in 0..col_name.len() {
                    hline = format!("{}-", hline);
                }
                hline_string = format!("{}{}+", hline_string, hline);
            }
            println!("{}", col_string);
            println!("{}", hline_string);
            for d in &self.data {
                let mut col_string = String::from("|");
                for col_name in &self.col_name_vec {
                    let mut value = d.get(&col_name);
                    let need_pad_len = (self.max_col_len[col_name] - value.len()) as i32;
                    for i in 0..need_pad_len {
                        if i % 2 == 0 {
                            value = format!("{} ", value);
                        } else {
                            value = format!(" {}", value);
                        }
                    }
                    col_string = format!("{}{}|", col_string, value);
                }
                println!("{}", col_string);
            }
        }
    }
}

fn recv_input(debug: bool) -> String {
    let mut command = String::new();
    "Please input a sql statement:".to_string().info_message();
    let b1 = std::io::stdin().read_line(&mut command).unwrap();
    let read_bytes = format!("read {} bytes", b1);
    read_bytes.remove_tails().debug_message(debug);
    command.remove_tails()
}

async fn mysql_query(conn: &mut MySqlConnection, sql: &str) -> anyhow::Result<()> {
    let rows = sqlx::query(sql).fetch_all(conn).await?;
    let mut sql_datas = SqlDatas::new();

    for rec in &rows {
        let mut sql_data = SqlData::new();
        // println!("{:?}", rec);
        let len = rec.len();
        // println!("{:?}", len);
        for i in 0..len {
            let col = rec.column(i);
            let col_name = col.name();
            sql_datas.push_col_name(col_name);
            // println!("{:?}", col);
            let type_info = col.type_info();
            match type_info.name() {
                "CHAR" | "VARCHAR" | "BLOB" | "TEXT" | "TINYBLOB" | "TINYTEXT" | "MEDIUMBLOB"
                | "MEDIUMTEXT" | "LONGBLOB" | "LONGTEXT" | "ENUM" => {
                    let value: String = rec.get(i);
                    // println!("{}", value);
                    sql_data.string_type.insert(col_name.to_string(), value);
                }
                "INT" | "TINYINT" | "SMALLINT" | "MEDIUMINT" | "BIGINT" => {
                    let value: i32 = rec.get(i);
                    // println!("{}", value);
                    sql_data.int_type.insert(col_name.to_string(), value);
                }
                "FLOAT" | "DOUBLE" | "DECIMAL" => {
                    let value: f32 = rec.get(i);
                    // println!("{}", value);
                    sql_data.float_type.insert(col_name.to_string(), value);
                }
                "DATE" | "DATETIME" | "TIMESTAMP" | "TIME" | "YEAR" => {
                    let value: chrono::DateTime<chrono::Utc> = rec.get(i);
                    // println!("{}", value);
                    sql_data.date_type.insert(col_name.to_string(), value);
                }
                _ => {
                    println!("Unsupported type: {}", type_info.name());
                    let value: Vec<u8> = rec.get(i);
                    sql_data.unsupport_type.insert(col_name.to_string(), value);
                }
            };
        }
        sql_datas.push_sql_data(sql_data);
    }

    sql_datas.show();

    Ok(())
}

#[tokio::main]
async fn mysql_connect(url: &str) -> Result<(), sqlx::Error> {
    // let url = "mysql://root:password@localhost:3307/db_name";
    // let url = "mariadb://root:password@localhost:3307/db_name";
    let info = format!("MySQL connecting to {}...", &url);
    info.info_message();
    // let pool = MySqlPool::connect(&url).await?;
    let mut conn = MySqlConnection::connect(&url).await?;
    loop {
        let command = recv_input(false);
        match command.as_str() {
            "exit" => return Ok(()),
            _ => {
                // conn.execute(command.as_str()).await?;
                // pool.execute(sqlx::query("select * from test")).await?;
                match mysql_query(&mut conn, &command).await {
                    Ok(_) => (),
                    Err(e) => {
                        let e_str = format!("Query error: {}", e);
                        e_str.error_message();
                    }
                };
            }
        };
        // println!("{:?}", result);
    }
}

async fn psql_query(conn: &mut PgConnection, sql: &str) -> anyhow::Result<()> {
    let rows = sqlx::query(sql).fetch_all(conn).await?;
    let mut sql_datas = SqlDatas::new();

    for rec in &rows {
        let mut sql_data = SqlData::new();
        // println!("{:?}", rec);
        let len = rec.len();
        // println!("{:?}", len);
        for i in 0..len {
            let col = rec.column(i);
            let col_name = col.name();
            sql_datas.push_col_name(col_name);
            // println!("{:?}", col);
            let type_info = col.type_info();
            match type_info.name() {
                "CHAR" | "VARCHAR" | "BLOB" | "TEXT" | "TINYBLOB" | "TINYTEXT" | "MEDIUMBLOB"
                | "MEDIUMTEXT" | "LONGBLOB" | "LONGTEXT" | "ENUM" => {
                    let value: String = rec.get(i);
                    // println!("{}", value);
                    sql_data.string_type.insert(col_name.to_string(), value);
                }
                "INT" | "TINYINT" | "SMALLINT" | "MEDIUMINT" | "BIGINT" => {
                    let value: i32 = rec.get(i);
                    // println!("{}", value);
                    sql_data.int_type.insert(col_name.to_string(), value);
                }
                "FLOAT" | "DOUBLE" | "DECIMAL" => {
                    let value: f32 = rec.get(i);
                    // println!("{}", value);
                    sql_data.float_type.insert(col_name.to_string(), value);
                }
                "DATE" | "DATETIME" | "TIMESTAMP" | "TIME" | "YEAR" => {
                    let value: chrono::DateTime<chrono::Utc> = rec.get(i);
                    // println!("{}", value);
                    sql_data.date_type.insert(col_name.to_string(), value);
                }
                _ => {
                    println!("Unsupported type: {}", type_info.name());
                }
            };
        }
        sql_datas.push_sql_data(sql_data);
    }

    sql_datas.show();

    Ok(())
}

#[tokio::main]
async fn psql_connect(url: &str) -> Result<(), sqlx::Error> {
    // let url = "postgres://postgres:password@localhost/test";
    let info = format!("PostgreSQL connecting to {}...", &url);
    info.info_message();
    // let pool = MySqlPool::connect(&url).await?;
    let mut conn = PgConnection::connect(&url).await?;
    loop {
        let command = recv_input(false);
        match command.as_str() {
            "exit" => return Ok(()),
            _ => {
                // conn.execute(command.as_str()).await?;
                // pool.execute(sqlx::query("select * from test")).await?;
                match psql_query(&mut conn, &command).await {
                    Ok(_) => (),
                    Err(e) => {
                        let e_str = format!("Query error: {}", e);
                        e_str.error_message();
                    }
                };
            }
        };
        // println!("{:?}", result);
    }
}

pub fn run(sqlurl: &str, sqltype: &str) {
    // let url = "mysql://root:password@localhost:3306/db_name";
    match sqltype {
        "psql" => match psql_connect(&sqlurl) {
            Ok(_) => (),
            Err(e) => println!("Exec sql failed: {}", e),
        },
        _ => match mysql_connect(&sqlurl) {
            Ok(_) => (),
            Err(e) => println!("Exec sql failed: {}", e),
        },
    }
}
