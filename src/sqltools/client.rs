use crate::Message;
use sqlx::Executor;
use sqlx::MySqlPool;
use sqlx::Connection;
use sqlx::MySqlConnection;

fn recv_input(debug: bool) -> String {
    let mut command = String::new();
    "Please input a sql statement :".to_string().info_message();
    let b1 = std::io::stdin().read_line(&mut command).unwrap();
    let read_bytes = format!("read {} bytes", b1);
    read_bytes.remove_tails().debug_message(debug);
    command.remove_tails()
}

#[tokio::main]
async fn mysql_connect(url: &str) -> Result<(), sqlx::Error> {
    // let url = "mysql://root:password@localhost:3307/db_name";
    // let url = "postgres://postgres:password@localhost/test";
    let info = format!("MySQL connecting to {}...", &url);
    info.info_message();
    // let pool = MySqlPool::connect(&url).await?;
    let mut conn = MySqlConnection::connect(&url).await?;
    loop {
        let command = recv_input(false);
        let result = match command.as_str() {
            "exit" => return Ok(()),
            _ => {
                conn.execute(command.as_str()).await?;
                // pool.execute(sqlx::query("select * from test")).await?;
            },
        };
        println!("{:?}", result);
    }
}

pub fn run(username: &str, password: &str, host: &str, port: &str, database: &str) {
    // let url = "mysql://root:password@localhost:3306/db_name";
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );
    // test
    let url = "mysql://test:justdoit@192.168.194.135:3306/rust";
    match mysql_connect(&url) {
        Ok(_) => (),
        Err(e) => println!("Exec sql failed: {}", e),
    }
}
