use mysql::prelude::*;
use mysql::*;

fn connect(url: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;

    Ok(())
}

pub fn run(username: &str, password: &str, host: &str, port: &str, database: &str) {
    // let url = "mysql://root:password@localhost:3307/db_name";
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );
    match connect(&url) {
        Ok(_) => (),
        Err(e) => println!("Exec sql failed: {}", e),
    }
}
