use chrono::Local;
use clap::Parser;
use colored::Colorize;

/// HackerOnEx tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

trait Message {
    fn wm(&self);
    fn im(&self);
    fn em(&self);
    fn dm(&self, debug: bool);
    fn rt(&self) -> String;
    fn aa(&self);
}

impl Message for String {
    fn wm(&self) {
        println!("{} {}", "[warning]".yellow(), self);
    }
    fn im(&self) {
        println!("{} {}", "[info]".green(), self);
    }
    fn em(&self) {
        println!("{} {}", "[error]".red(), self);
    }
    fn dm(&self, debug: bool) {
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
    fn aa(&self) {
        let date = Local::now();
        let date_str = date.format("%Y-%m-%d %H:%M:%S");
        println!("{} [{}] [{}]", ARROW.bold().green(), self, date_str);
    }
}

mod search;
mod watchdog;

const WELCOME_INFO: &str = r"
 _   _            _             _____       _____     
| | | |          | |           |  _  |     |  ___|    
| |_| | __ _  ___| | _____ _ __| | | |_ __ | |____  __
|  _  |/ _` |/ __| |/ / _ \ '__| | | | '_ \|  __\ \/ /
| | | | (_| | (__|   <  __/ |  \ \_/ / | | | |___>  < 
\_| |_/\__,_|\___|_|\_\___|_|   \___/|_| |_\____/_/\_\


";

const ARROW: &str = ">>>";

fn search(debug: bool) {
    loop {
        "search".to_string().aa();
        
        let mut command = String::new();
        let b1 = std::io::stdin().read_line(&mut command).unwrap();
        command.rt().dm(debug);
        let read_bytes = format!("read {} bytes", b1);
        read_bytes.rt().dm(debug);

        match command.rt().as_str() {
            "filestag" | "fs" => watchdog::filestag::run("test", debug, 1.0),
            "list" | "ls" => println!("filestag"),
            _ => invaild_command(debug),
        }
    }
}

fn watchdog(debug: bool) {
    "watchdog".to_string().aa();
}

fn invaild_command(debug: bool) {
    "invaild command".to_string().wm();
}

fn main() {
    let args = Args::parse();
    let debug = args.debug;
    println!("{}", WELCOME_INFO.bold().red());
    loop {
        "main".to_string().aa();
        let mut command = String::new();
        let b1 = std::io::stdin().read_line(&mut command).unwrap();
        command.rt().dm(debug);
        let read_bytes = format!("read {} bytes", b1);
        read_bytes.rt().dm(debug);

        match command.rt().as_str() {
            "search" => search(debug),
            "watchdog" => watchdog(debug),
            _ => invaild_command(debug),
        }
    }
    /*
    let args = Args::parse();
    match args.tool.as_str() {
        "watchdog" => watchdog::filestag::run(&args.filestag, args.debug, args.delay),
        _ => search::exploitalert::search(&args.name),
    }
    */
}
