use rssql::{MySQL, PostgreSQL};
use anyhow::Result;

use crate::recv_input;
use crate::Message;


async fn psql_connect(url: &str) -> Result<()> {
    // let url = "mysql://root:password@localhost:3307/db_name";
    // let url = "mariadb://root:password@localhost:3307/db_name";
    let info = format!("PostgreSQL connecting to {}...", &url);
    info.info_message();
    // let pool = MySqlPool::connect(&url).await?;
    let mut psql = PostgreSQL::connect(url).await.unwrap();
    match psql.check_connection().await {
        true => {
            loop {
                let command = recv_input();
                match command.as_str() {
                    "exit" => {
                        psql.close().await;
                        return Ok(());
                    }
                    _ => match psql.execute(&command).await {
                        Ok(_) => (),
                        Err(e) => {
                            let e_str = format!("Query error: {}", e);
                            e_str.error_message();
                        }
                    },
                };
                // println!("{:?}", result);
            }
        }
        false => {
            "Connect to database failed".to_string().error_message();
            Ok(())
        }
    }
}

async fn mysql_connect(url: &str) -> Result<()> {
    // let url = "mysql://root:password@localhost:3307/db_name";
    // let url = "mariadb://root:password@localhost:3307/db_name";
    let info = format!("MySQL connecting to {}...", &url);
    info.info_message();
    // let pool = MySqlPool::connect(&url).await?;
    let mut mysql = MySQL::connect(url).await.unwrap();
    match mysql.check_connection().await {
        true => {
            loop {
                let command = recv_input();
                match command.as_str() {
                    "exit" => {
                        mysql.close().await;
                        return Ok(());
                    }
                    _ => match mysql.execute(&command).await {
                        Ok(_) => (),
                        Err(e) => {
                            let e_str = format!("Query error: {}", e);
                            e_str.error_message();
                        }
                    },
                };
                // println!("{:?}", result);
            }
        }
        false => {
            "Connect to database failed".to_string().error_message();
            Ok(())
        }
    }
}

pub async fn run(sqlurl: &str) {
    // NOTE: SQLite is only have C API, and MSSQL not support fully in SQLx
    // let url = "mysql://root:password@localhost:3306/db_name";
    let sqlurl_split: Vec<&str> = sqlurl.split(":").collect();
    if sqlurl_split.len() > 0 {
        let sqltype = sqlurl_split[0];
        match sqltype {
            "postgres" => match psql_connect(&sqlurl).await {
                Ok(_) => (),
                Err(e) => println!("Exec sql failed: {}", e),
            },
            "mysql" | "mariadb" => match mysql_connect(&sqlurl).await {
                Ok(_) => (),
                Err(e) => println!("Exec sql failed: {}", e),
            },
            _ => {
                let e_str = format!("Wrong database type: {}", sqltype);
                e_str.error_message();
            }
        }
    } else {
        let e_str = format!("Wrong database url: {}", sqlurl);
        e_str.error_message();
    }
}
