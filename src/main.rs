use chrono::Local;
use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;

mod brute;
mod search;
mod watchdog;

const VERSION: &str = "v0.2.0";

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
        let result = if self.contains("\r\n") {
            match self.strip_suffix("\r\n") {
                Some(m) => m.to_string(),
                None => self.to_string(),
            }
        } else if self.contains("\n") {
            match self.strip_suffix("\n") {
                Some(m) => m.to_string(),
                None => self.to_string(),
            }
        } else {
            self.to_string()
        };
        result
    }
    fn arrow(&self) {
        let date = Local::now();
        let date_str = date.format("%Y-%m-%d %H:%M:%S");
        println!("{} {} [{}]", ">".green(), self.green(), date_str);
    }
    fn invaild_command(&self) {
        let error_message = format!("??? --> {}", self);
        error_message.warning();
    }
}

struct Parameters {
    str_parameters: HashMap<String, Option<String>>,
    bool_parameters: HashMap<String, bool>,
}

impl Parameters {
    fn new() -> Parameters {
        Parameters {
            str_parameters: HashMap::new(),
            bool_parameters: HashMap::new(),
        }
    }
    fn get_str(&self, name: &str) -> Option<String> {
        match self.str_parameters.get(name) {
            Some(a) => (*a).clone(),
            _ => None,
        }
    }
    fn get_bool(&self, name: &str) -> bool {
        match self.bool_parameters.get(name) {
            Some(a) => *a,
            _ => false,
        }
    }
    fn add_str(&mut self, key: &str, value: Option<String>) {
        self.str_parameters.insert(key.to_string(), value);
    }
    fn add_bool(&mut self, key: &str, value: bool) {
        self.bool_parameters.insert(key.to_string(), value);
    }
}

struct CommandsMap {
    long: String,
    short: String,
    f: fn(&mut Parameters),
    require_parameters: bool,
    parameters: Vec<String>,
}

struct Commands<'a> {
    name: &'a str,
    level: usize,
    map: Vec<CommandsMap>,
}

impl Commands<'_> {
    fn new<'a>(name: &'a str, level: usize) -> Commands<'a> {
        Commands {
            name,
            level,
            map: Vec::new(),
        }
    }
    fn add(
        &mut self,
        long: &str,
        short: &str,
        f: fn(&mut Parameters),
        require_parameters: bool,
        parameters: Vec<&str>,
    ) {
        let map = CommandsMap {
            long: long.to_string(),
            short: short.to_string(),
            f,
            require_parameters,
            parameters: parameters.into_iter().map(|s| s.to_string()).collect(),
        };
        self.map.push(map);
    }
    fn menu(&self) {
        // println!("{}", self.map.len());
        for m in &self.map {
            println!("> {}({})", m.long.red(), m.short.red());
        }
    }
    fn run(&self, p: &mut Parameters) {
        fn get_more_parameters(p: &mut Parameters, vec_string: &Vec<String>) {
            let debug = p.get_bool("debug");
            for s in vec_string {
                let info_str = format!("> Please input [{}] value...", s);
                println!("{}", info_str.green());
                let input = recv_input(debug);
                p.add_str(&s, Some(input));
            }
        }
        let debug = p.get_bool("debug");
        loop {
            self.name.to_string().arrow();
            let inputs = recv_input(debug);
            // println!("inputs: {}", inputs);
            let mut match_command = false;
            if inputs == "list" || inputs == "ls" {
                Self::menu(&self);
                match_command = true;
            } else if inputs == "back" || inputs == "b" {
                if self.level != 0 {
                    // top level can not back anymore
                    break;
                } else {
                    match_command = true;
                    "Please use ctrl-c to exit program".to_string().warning();
                }
            } else if inputs.rt().len() == 0 {
                match_command = true;
            } else {
                for m in &self.map {
                    if inputs == m.long || (inputs == m.short && m.short != "null") {
                        if m.require_parameters {
                            get_more_parameters(p, &m.parameters);
                        }
                        (m.f)(p);
                        match_command = true;
                    } else {
                    }
                }
            }
            if match_command == false {
                inputs.invaild_command();
            }
            // println!();
        }
    }
}

/* FUNCTION */

fn recv_input(debug: bool) -> String {
    let mut command = String::new();
    let b1 = std::io::stdin().read_line(&mut command).unwrap();
    command.rt().debug(debug);
    let read_bytes = format!("read {} bytes", b1);
    read_bytes.rt().debug(debug);
    command.rt()
}

fn search(p: &mut Parameters) {
    fn run_exploitalert(p: &mut Parameters) {
        // let debug = p.get_bool("debug");
        let proxy = p.get_str("proxy");
        let name = p.get_str("name").unwrap();
        "running...".to_string().info();
        search::exploitalert::run(&name, &proxy);
        "finish".to_string().info();
    }

    let mut commands = Commands::new("search", 1);
    commands.add("exploitalert", "ea", run_exploitalert, true, vec!["name"]);
    commands.run(p);
}

fn watchdog(p: &mut Parameters) {
    fn run_filestag(p: &mut Parameters) {
        let debug = p.get_bool("debug");
        let path = p.get_str("path").unwrap();
        let delay = p.get_str("delay").unwrap();
        let delay: f32 = delay.parse().unwrap();
        watchdog::filestag::run(&path, debug, delay);
    }

    let mut commands = Commands::new("watchdog", 1);
    commands.add("filestag", "fs", run_filestag, true, vec!["path", "delay"]);
    commands.run(p);
}

fn brute(p: &mut Parameters) {
    fn run_webdir(p: &mut Parameters) {
        let path = p
            .get_str("wordlists_path (press enter to use default wordlists, or 'all' to use all wordlists)")
            .unwrap();
        let target = p.get_str("target").unwrap();
        // test
        // let path = "./src/brute/wordlists/common.txt";
        // let target = "http://192.168.194.131/";
        if target.len() != 0 {
            if path.len() == 0 {
                let wordlists = include_str!("./brute/wordlists/common.txt");
                // let wordlists = include_bytes!("./brute/wordlists/big.txt");
                // let wordlists = String::from_utf8_lossy(wordlists);
                brute::webdir::run(&path, &target, Some(&wordlists));
            } else {
                if path == "all" {
                    let wordlists = include_bytes!("./brute/wordlists/de_all.txt");
                    let wordlists = String::from_utf8_lossy(wordlists);
                    brute::webdir::run(&path, &target, Some(&wordlists));
                } else {
                    brute::webdir::run(&path, &target, None);
                }
            }
        } else {
            "target should not be null".to_string().error();
        }
    }

    fn run_portscan(p: &mut Parameters) {
        let target = p.get_str("target").unwrap();
        // i.e. 22-100
        let port_range = p.get_str("port_range(22-1024)").unwrap();
        let mut protocol = p.get_str("protocol(press enter to use tcp)").unwrap();
        if protocol.len() == 0 {
            protocol = "tcp".to_string();
        }
        brute::portscan::run(&target, &port_range, &protocol);
    }

    let mut commands = Commands::new("brute", 1);
    commands.add(
        "webdir",
        "wr",
        run_webdir,
        true,
        vec![
            "wordlists_path (press enter to use default wordlists, or 'all' to use all wordlists)",
            "target",
        ],
    );
    commands.add(
        "portscan",
        "ps",
        run_portscan,
        true,
        vec![
            "target",
            "port_range(22-1024)",
            "protocol(press enter to use tcp)",
        ],
    );
    commands.run(p);
}

fn main() {
    ctrlc::set_handler(move || {
        "bye~".to_string().info();
        std::process::exit(0);
    })
    .expect("set ctrlc failed");

    let args = Args::parse();
    let debug = args.debug;
    let proxy: Option<String> = match args.proxy.as_str() {
        "null" => None,
        _ => Some(args.proxy.to_string()),
    };
    println!("{}\n{}", WELCOME_INFO.bold().red(), VERSION.bold().green());
    // Parameters
    let mut p = Parameters::new();
    p.add_bool("debug", debug);
    p.add_str("proxy", proxy);
    // Commands
    let mut commands = Commands::new("main", 0);
    commands.add("search", "sr", search, false, vec![]);
    commands.add("watchdog", "wd", watchdog, false, vec![]);
    commands.add("brute", "bt", brute, false, vec![]);
    commands.run(&mut p);
}
