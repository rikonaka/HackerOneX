use chrono::Local;
use clap::Parser;
use colored::Colorize;

const ARROW: &str = ">>>";

const WELCOME_INFO: &str = r"
 _   _            _             _____       _____     
| | | |          | |           |  _  |     |  ___|    
| |_| | __ _  ___| | _____ _ __| | | |_ __ | |____  __
|  _  |/ _` |/ __| |/ / _ \ '__| | | | '_ \|  __\ \/ /
| | | | (_| | (__|   <  __/ |  \ \_/ / | | | |___>  < 
\_| |_/\__,_|\___|_|\_\___|_|   \___/|_| |_\____/_/\_\


";

/// HackerOnEx tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
    /// Set proxy
    #[arg(short, long, default_value = "null")] // socks5://127.0.0.1:1080
    proxy: String,
}

struct Menu<'a> {
    name: &'a str,
    data: Vec<Vec<&'a str>>,
}

impl Menu<'_> {
    fn list(&self) {
        for d in &self.data {
            println!(
                "{} [{}] - {}({})",
                ">".green(),
                self.name,
                d[0].green(),
                d[1].green()
            );
        }
    }
}

fn recv_input(debug: bool) -> String {
    let mut command = String::new();
    let b1 = std::io::stdin().read_line(&mut command).unwrap();
    command.rt().debug(debug);
    let read_bytes = format!("read {} bytes", b1);
    read_bytes.rt().debug(debug);
    command.rt()
}

fn search(debug: bool, proxy: &Option<String>) {
    let menu = Menu {
        name: "search",
        data: vec![vec!["exploitalert", "ea"]], // long, short
    };
    fn run_exploitalert(debug: bool, proxy: &Option<String>) {
        println!("{}", "> Please input name of exploit.".green());
        let name = recv_input(debug);
        "running...".to_string().info();
        search::exploitalert::run(&name, proxy);
        "finish".to_string().info();
    }
    loop {
        "search".to_string().arrow();
        let command = recv_input(debug);
        match command.as_str() {
            "exploitalert" | "ea" => run_exploitalert(debug, proxy),
            "list" | "ls" => menu.list(),
            "back" | "b" => break,
            _ => command.invaild_command(),
        }
    }
}

fn watchdog(debug: bool) {
    loop {
        "watchdog".to_string().arrow();
        let command = recv_input(debug);
        match command.as_str() {
            "filestag" | "fs" => watchdog::filestag::run("test", debug, 1.0),
            "list" | "ls" => println!("filestag"),
            "back" | "b" => break,
            _ => command.invaild_command(),
        }
    }
}

trait Message {
    fn warning(&self);
    fn info(&self);
    fn error(&self);
    fn debug(&self, debug: bool);
    fn rt(&self) -> String;
    fn arrow(&self);
    fn invaild_command(&self);
}

impl Message for String {
    fn warning(&self) {
        println!("{} {}", "[warning]".yellow(), self);
    }
    fn info(&self) {
        println!("{} {}", "[info]".green(), self);
    }
    fn error(&self) {
        println!("{} {}", "[error]".red(), self);
    }
    fn debug(&self, debug: bool) {
        match debug {
            true => println!("{} {}", "[debug]".yellow(), self),
            _ => (),
        }
    }
    fn rt(&self) -> String {
        match self.strip_suffix("\n") {
            Some(m) => m.to_string(),
            None => self.to_string(),
        }
    }
    fn arrow(&self) {
        let date = Local::now();
        let date_str = date.format("%Y-%m-%d %H:%M:%S");
        println!("{} [{}] [{}]", ARROW.bold().green(), self, date_str);
    }
    fn invaild_command(&self) {
        let error_message = format!("??? --> {}", self);
        error_message.warning();
    }
}

mod search;
mod watchdog;

fn main() {
    let args = Args::parse();
    let debug = args.debug;
    let proxy: Option<String> = match args.proxy.as_str() {
        "null" => None,
        _ => Some(args.proxy.to_string()),
    };
    println!("{}", WELCOME_INFO.bold().red());
    let menu = Menu {
        name: "main",
        data: vec![vec!["search", "sr"], vec!["watchdog", "wd"]],
    };

    loop {
        "main".to_string().arrow();
        let command = recv_input(debug);
        match command.as_str() {
            "search" | "sr" => search(debug, &proxy),
            "watchdog" | "wd" => watchdog(debug),
            "list" | "ls" => menu.list(),
            _ => command.invaild_command(),
        }
    }
}
