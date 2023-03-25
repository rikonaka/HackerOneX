use crate::Message;
use sqlx::types::chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::types::{BigDecimal, JsonValue, Uuid};
// use sqlx::postgres::types::PgInterval;
// use sqlx::postgres::types::PgRange;
// use sqlx::postgres::types::PgMoney;
// use sqlx::postgres::types::PgTimeTz;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::types::mac_address::MacAddress;
// use sqlx::types::BitVec;
use sqlx::{Column, Connection, Row, TypeInfo};
use sqlx::{MySqlConnection, PgConnection};
use std::collections::HashMap;
// use std::error::Error;

use crate::NULL;

struct SqlRow {
    max_len: usize,
    row: HashMap<String, String>,
}

impl SqlRow {
    fn new() -> SqlRow {
        let row = HashMap::new();
        SqlRow { max_len: 18, row }
    }
    fn get(&self, col_name: &str) -> String {
        let value = if self.row.contains_key(col_name) {
            match self.row.get(col_name) {
                Some(v) => v.to_string(),
                None => format!("[{}]", NULL),
            }
        } else {
            format!("[{}]", NULL)
        };
        value
    }
    fn insert(&mut self, col_name: &str, value: String) {
        let mix_value = if value.len() > self.max_len {
            let mut value_part = value[..self.max_len].to_string();
            value_part.push_str("..");
            value_part
        } else {
            value
        };
        self.row.insert(col_name.to_string(), mix_value);
    }
}

struct SqlDatas {
    col_name_vec: Vec<String>,
    max_col_len: HashMap<String, usize>,
    sql_row: Vec<SqlRow>,
}

impl SqlDatas {
    fn new() -> SqlDatas {
        let max_col_len = HashMap::new();
        let col_name_vec = Vec::new();
        let data: Vec<SqlRow> = Vec::new();
        SqlDatas {
            col_name_vec,
            max_col_len,
            sql_row: data,
        }
    }
    fn push_sql_data(&mut self, sql_data: SqlRow) {
        self.sql_row.push(sql_data);
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
        for d in &self.sql_row {
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
            for d in &self.sql_row {
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

fn unsupported_type(name: &str) {
    let e_str = format!("Unsupported type: {}", name);
    e_str.warning_message();
}

fn recv_input() -> String {
    let mut command = String::new();
    "Please input a sql statement:".to_string().info_message();
    let _ = std::io::stdin().read_line(&mut command).unwrap();
    // let b1 = std::io::stdin().read_line(&mut command).unwrap();
    // let read_bytes = format!("read {} bytes", b1);
    // read_bytes.remove_tails().debug_message(debug);
    command.remove_tails()
}

async fn mysql_query(conn: &mut MySqlConnection, sql: &str) -> anyhow::Result<()> {
    let rows = sqlx::query(sql).fetch_all(conn).await?;
    let mut sql_datas = SqlDatas::new();

    for rec in &rows {
        let mut sql_data = SqlRow::new();
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
                "BOOLEAN" | "TINYINT(1)" => {
                    let value: bool = rec.get(i);
                    sql_data.row.insert(col_name.to_string(), value.to_string());
                }
                "TINYINT" => {
                    let value: i8 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "SMALLINT" => {
                    let value: i16 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "INT" => {
                    let value: i32 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BIGINT" => {
                    let value: i64 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TINYINT UNSIGNED" => {
                    let value: u8 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "SMALLINT UNSIGNED" => {
                    let value: u16 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "INT UNSIGNED" => {
                    let value: u32 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BIGINT UNSIGNED" => {
                    let value: u64 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "FLOAT" => {
                    let value: f32 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DOUBLE" => {
                    let value: f64 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "VARCHAR" | "CHAR" | "TEXT" => {
                    let value: String = rec.get(i);
                    // println!("{}", value);
                    sql_data.insert(col_name, value);
                }
                "VARBINARY" | "BINARY" | "BLOB" => {
                    // let value: Vec<u8> = rec.get(i);
                    sql_data.insert(col_name, "[binary]".to_string());
                }
                "TIMESTAMP" => {
                    let value: DateTime<chrono::Utc> = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DATETIME" => {
                    let value: NaiveDateTime = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DATE" => {
                    let value: NaiveDate = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TIME" => {
                    let value: NaiveTime = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DECIMAL" => {
                    let value: BigDecimal = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BYTE(16)" => {
                    let value: Uuid = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "JSON" => {
                    let value: JsonValue = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                _ => {
                    // let value: Vec<u8> = rec.get(i);
                    unsupported_type(type_info.name());
                    sql_data
                        .row
                        .insert(col_name.to_string(), "[binary]".to_string());
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
        let command = recv_input();
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
        let mut sql_data = SqlRow::new();
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
                "BOOL" => {
                    let value: bool = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "CHAR" => {
                    let value: i8 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "SMALLINT" | "SMALLSERIAL" | "INT2" => {
                    let value: i16 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "INT" | "SERIAL" | "INT4" => {
                    let value: i32 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BIGINT" | "BIGSERIAL" | "INT8" => {
                    let value: i64 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "REAL" | "FLOAT4" => {
                    let value: f32 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DOUBLE PRECISION" | "FLOAT8" => {
                    let value: f64 = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "VARCHAR" | "CHAR(N)" | "TEXT" | "NAME" => {
                    let value: String = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BYTEA" => {
                    // let value: Vec<u8> = rec.get(i);
                    sql_data.insert(col_name, "<binary>".to_string());
                }
                "INTERVAL" => {
                    // let value: PgInterval = rec.get(i);
                    sql_data.insert(col_name, "<interval>".to_string());
                }
                "INT8RANGE" | "INT4RANGE" | "TSRANGE" | "TSTZRANGE" | "DATERANGE" | "NUMRANGE" => {
                    // let value: PgRange<T> = rec.get(i);
                    sql_data.insert(col_name, "<range>".to_string());
                }
                "MONEY" => {
                    // let value: PgMoney = rec.get(i);
                    sql_data.insert(col_name, "<money>".to_string());
                }
                "NUMERIC" => {
                    let value: BigDecimal = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TIMESTAMPTZ" => {
                    let value: DateTime<chrono::Utc> = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TIMESTAMP" => {
                    let value: NaiveDateTime = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "DATE" => {
                    let value: NaiveDate = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TIME" => {
                    let value: NaiveTime = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "TIMETZ" => {
                    // let value: PgTimeTz = rec.get(i);
                    sql_data.insert(col_name, "<timez>".to_string());
                }
                "UUID" => {
                    let value: Uuid = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "INET" | "CIDR" => {
                    let value: IpNetwork = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "MACADDR" => {
                    let value: MacAddress = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                "BIT" | "VARBIT" => {
                    // let value: BitVec = rec.get(i);
                    sql_data.insert(col_name, "<bit>".to_string());
                }
                "JSON" | "JSONB" => {
                    let value: JsonValue = rec.get(i);
                    sql_data.insert(col_name, value.to_string());
                }
                _ => {
                    unsupported_type(type_info.name());
                    // let value: Vec<u8> = rec.get(i);
                    sql_data.insert(col_name, "[binary]".to_string());
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
        let command = recv_input();
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

// #[tokio::main]
// async fn sqlite_connect(url: &str) -> Result<(), Box<dyn Error>> {
//     let info = format!("SQLite connecting to {}...", &url);
//     info.info_message();
//     let conn = sqlite::open(url).unwrap();
//     loop {
//         let command = recv_input();
//         match command.as_str() {
//             "exit" => return Ok(()),
//             _ => {
//                 conn.execute(command).unwrap();
//             }
//         };
//     }
// }

pub fn run(sqlurl: &str) {
    // NOTE: SQLite is only have C API, and MSSQL not support fully in SQLx
    // let url = "mysql://root:password@localhost:3306/db_name";
    let sqlurl_split = sqlurl.split(":");
    let sqlurl_vec: Vec<&str> = sqlurl_split.collect();
    if sqlurl_vec.len() > 0 {
        let sqltype = sqlurl_vec[0];
        match sqltype {
            "postgres" => match psql_connect(&sqlurl) {
                Ok(_) => (),
                Err(e) => println!("Exec sql failed: {}", e),
            },
            "mysql" | "mariadb" => match mysql_connect(&sqlurl) {
                Ok(_) => (),
                Err(e) => println!("Exec sql failed: {}", e),
            },
            // "sqlite" => match sqlite_connect(&sqlurl) {
            //     Ok(_) => (),
            //     Err(e) => println!("Exec sql failed: {}", e),
            // },
            _ => {
                let e_str = format!("Wrong database type: {}", sqltype);
                e_str.error_message();
            }
        }
    }
}
