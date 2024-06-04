use chrono::Local;
use clap::Parser;
use colored::Colorize;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

mod brute;
mod honeypot;
mod search;
mod sqltools;
mod watchdog;

static LOG_FLAG: OnceCell<bool> = OnceCell::new();
static VERBOSE_FLAG: OnceCell<bool> = OnceCell::new();

const NULL_VALUE: &str = "null";

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
    /// Set proxy
    #[arg(short, long, default_value=NULL_VALUE)] // socks5://127.0.0.1:1080
    proxy: String,
    /// Log to file
    #[arg(short, long, action)]
    log: bool,
    /// Level 0 command
    #[arg(long, default_value=NULL_VALUE)]
    level0: String,
    /// Level 1 command
    #[arg(long, default_value=NULL_VALUE)]
    level1: String,
    /// Level 2 command
    #[arg(long, default_value=NULL_VALUE)]
    level2: String,
    /// Set in verbose mode
    #[arg(short, long, action)]
    verbose: bool,
}

trait Message {
    fn log_to_file(&self);
    fn warning_message(&self);
    fn info_message(&self);
    fn get_info_message(&self) -> String;
    fn error_message(&self);
    fn verbose_message(&self);
    fn arrow_message(&self);
    fn invaild_command(&self);
}

// macro_rules! warning_message {
//     () => {
//         warning_message(True)
//     };
// }

impl Message for String {
    fn log_to_file(&self) {
        let log = LOG_FLAG.get().expect("Get global value failed");
        match log {
            true => {
                let date = Local::now();
                let date_str = date.format("%Y-%m-%d %H:%M:%S");
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open("./hackeronex.log")
                    .unwrap();
                writeln!(file, "{} - {}", date_str, self).expect("Write to log file failed");
            }
            _ => (),
        }
    }
    fn warning_message(&self) {
        let message = format!("{} {}", "[warning]".yellow(), self);
        println!("{}", &message);
        let log_message = format!("{} {}", "[warning]", self);
        log_message.log_to_file();
    }
    fn info_message(&self) {
        let message = format!("{} {}", "[info]".green(), self);
        println!("{}", &message);
        let log_message = format!("{} {}", "[info]", self);
        log_message.log_to_file();
    }
    fn get_info_message(&self) -> String {
        format!("{} {}", "[info]".green(), self)
    }
    fn error_message(&self) {
        let message = format!("{} {}", "[error]".red(), self);
        println!("{}", &message);
        let log_message = format!("{} {}", "[error]", self);
        log_message.log_to_file();
    }
    fn verbose_message(&self) {
        let verbose = VERBOSE_FLAG.get().expect("Get global value failed");
        match verbose {
            true => {
                let message = format!("{} {}", "[verbose]".yellow(), self);
                println!("{}", message);
                let log_message = format!("{} {}", "[verbose]", self);
                log_message.log_to_file();
            }
            _ => (),
        }
    }
    fn arrow_message(&self) {
        let date = Local::now();
        let date_str = date.format("%Y-%m-%d %H:%M:%S");
        println!("{} {} [{}]", ">".green(), self.green(), date_str);
    }
    fn invaild_command(&self) {
        let error_message = format!("??? --> {}", self);
        error_message.warning_message();
    }
}

struct Parameters {
    str_parameters: HashMap<String, Option<String>>,
}

impl Parameters {
    fn new() -> Parameters {
        Parameters {
            str_parameters: HashMap::new(),
        }
    }
    fn get_str(&self, name: &str) -> Option<String> {
        match self.str_parameters.get(name) {
            Some(a) => (*a).clone(),
            _ => None,
        }
    }
    fn add_str(&mut self, key: &str, value: Option<String>) {
        self.str_parameters.insert(key.to_string(), value);
    }
}

struct CommandsMap {
    long: String,
    short: String,
    f: fn(&mut Parameters),
    require_parameters: bool,
    parameters: Vec<String>,
    default_value: Vec<String>,
    info_value: Vec<String>,
}

struct Commands<'a> {
    name: &'a str,
    level: usize,
    commandsmap: Vec<CommandsMap>,
}

impl Commands<'_> {
    fn new<'a>(name: &'a str, level: usize) -> Commands<'a> {
        Commands {
            name,
            level,
            commandsmap: Vec::new(),
        }
    }
    fn add(
        &mut self,
        long: &str,
        short: &str,
        f: fn(&mut Parameters),
        require_parameters: bool,
        parameters: Vec<&str>,
        default_parameters: Vec<&str>,
        info_value: Vec<&str>,
    ) {
        if (parameters.len() != default_parameters.len())
            | (parameters.len() != info_value.len())
            | (default_parameters.len() != info_value.len())
        {
            let e_str = "Commands add parameters should has same length".to_string();
            e_str.error_message();
            panic!("{}", e_str);
        }
        let map = CommandsMap {
            long: long.to_string(),
            short: short.to_string(),
            f,
            require_parameters,
            parameters: parameters.into_iter().map(|s| s.to_string()).collect(),
            default_value: default_parameters
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            info_value: info_value.into_iter().map(|s| s.to_string()).collect(),
        };
        self.commandsmap.push(map);
    }
    fn menu(&self) {
        // println!("{}", self.map.len());
        for map in &self.commandsmap {
            println!("> {}({})", map.long.red(), map.short.red());
        }
    }
    fn run(&self, p: &mut Parameters) {
        fn get_more_parameters(
            p: &mut Parameters,
            parameters_vec: &Vec<String>,
            default_vec: &Vec<String>,
            info_vec: &Vec<String>,
        ) {
            // let debug = p.get_bool("debug");
            for (i, s) in parameters_vec.iter().enumerate() {
                let info_str = format!(
                    "> Please input [{}] parameter ({}, default: {})...",
                    s, &info_vec[i], &default_vec[i]
                );
                println!("{}", info_str.green());
                let input = recv_input();
                if input.len() > 0 {
                    p.add_str(&s, Some(input));
                } else {
                    p.add_str(&s, Some(default_vec[i].clone()))
                }
            }
        }
        // let debug = p.get_bool("debug");
        loop {
            self.name.to_string().arrow_message();
            let inputs = recv_input();
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
                    "Please use ctrl-c to exit program"
                        .to_string()
                        .warning_message();
                }
            } else if inputs.trim().len() == 0 {
                match_command = true;
            } else {
                for map in &self.commandsmap {
                    if inputs == map.long || (inputs == map.short && map.short != NULL_VALUE) {
                        if map.require_parameters {
                            get_more_parameters(
                                p,
                                &map.parameters,
                                &map.default_value,
                                &map.info_value,
                            );
                        }
                        (map.f)(p);
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

fn recv_input() -> String {
    let mut command = String::new();
    let _ = std::io::stdin().read_line(&mut command).unwrap();
    // let b1 = std::io::stdin().read_line(&mut command).unwrap();
    // command.remove_tails().debug_message(debug);
    // let read_bytes = format!("read {} bytes", b1);
    // read_bytes.remove_tails().debug_message(debug);
    command.trim().to_string()
}

fn search(p: &mut Parameters) {
    fn run_exploitalert(p: &mut Parameters) {
        // let debug = p.get_bool("debug");
        let proxy = p.get_str("proxy");
        let name = p.get_str("name").unwrap();
        "running...".to_string().info_message();
        search::exploitalert::run(&name, &proxy);
        "finish".to_string().info_message();
    }

    let mut commands = Commands::new("search", 1);
    commands.add(
        "exploitalert",
        "ea",
        run_exploitalert,
        true,
        vec!["name"],
        vec!["discuz!"],
        vec!["name of the vulnerability you want to query"],
    );
    commands.run(p);
}

fn watchdog(p: &mut Parameters) {
    fn run_filestag(p: &mut Parameters) {
        let path = p.get_str("path").unwrap();
        let delay = p.get_str("delay").unwrap();
        let delay: f32 = delay.parse().unwrap();
        watchdog::filestag::run(&path, delay);
    }

    let mut commands = Commands::new("watchdog", 1);
    commands.add(
        "filestag",
        "fs",
        run_filestag,
        true,
        vec!["path", "delay"],
        vec!["./test/", "1.0"],
        vec!["the path you wanna watch", "how often to check for changes"],
    );
    commands.run(p);
}

fn brute(p: &mut Parameters) {
    fn run_webdir(p: &mut Parameters) {
        let path = p.get_str("wordlists_path").unwrap();
        let target = p.get_str("target").unwrap();
        // test
        // let path = "./src/brute/wordlists/common.txt";
        // let target = "http://192.168.194.131/";
        let wordlists = match path.as_str() {
            "common" => Some(include_str!("./brute/wordlists/common.txt")),
            "all" => Some(include_str!("./brute/wordlists/de_all.txt")),
            _ => None,
        };
        // let wordlists = include_bytes!("./brute/wordlists/big.txt");
        // let wordlists = String::from_utf8_lossy(wordlists);
        brute::webdir::run(&path, &target, wordlists);
    }

    fn run_portscan(p: &mut Parameters) {
        let target = p.get_str("target").unwrap();
        // i.e. 22-100
        let port_range = p.get_str("port_range").unwrap();
        let mut protocol = p.get_str("protocol").unwrap();
        if protocol.len() == 0 {
            protocol = "tcp".to_string();
        }
        brute::portscan::run(&target, &port_range, &protocol);
    }

    fn run_arpscan(p: &mut Parameters) {
        let subnet = p.get_str("subnet").unwrap();
        let interface = p.get_str("interface").unwrap();
        brute::arpscan::run(&subnet, &interface);
    }

    let mut commands = Commands::new("brute", 1);
    commands.add(
        "webdir",
        "wr",
        run_webdir,
        true,
        vec!["wordlists_path", "target"],
        vec!["common", "127.0.0.1"],
        vec!["wordlists file path", "scan target address"],
    );
    commands.add(
        "portscan",
        "ps",
        run_portscan,
        true,
        vec!["target", "port_range", "protocol"],
        vec!["127.0.0.1", "1-1023", "tcp"],
        vec!["port scan target address", "port range", "scan protocol"],
    );
    commands.add(
        "arpscan",
        "as",
        run_arpscan,
        true,
        vec!["subnet", "interface"],
        vec!["192.168.1.0/24", "eno1"],
        vec!["arp scan target subnet", "arp scan interface"],
    );
    commands.run(p);
}

fn sqltools(p: &mut Parameters) {
    fn run_client(p: &mut Parameters) {
        let sqlurl = p.get_str("sqlurl").unwrap();
        sqltools::client::run(&sqlurl);
    }

    let mut commands = Commands::new("sqltools", 1);
    commands.add(
        "client",
        "ct",
        run_client,
        true,
        vec!["sqlurl"],
        vec!["mysql://root:password@localhost:3306/db_name"],
        vec!["target sql url"],
    );
    commands.run(p);
}

fn honeypot(p: &mut Parameters) {
    fn run_web(p: &mut Parameters) {
        let address = p.get_str("address").unwrap();
        let port = p.get_str("port").unwrap();
        let config = p.get_str("config").unwrap();
        let port: u16 = port.parse().unwrap();
        honeypot::web::run(&address, port, &config);
    }

    let mut commands = Commands::new("honeypot", 1);
    commands.add(
        "web",
        "w",
        run_web,
        true,
        vec!["address", "port", "config"],
        vec!["0.0.0.0", "8080", "./src/honeypot/response.txt"],
        vec!["listen address", "listen port", "config file path"],
    );
    commands.run(p);
}

fn shell(_: &mut Parameters) {
    loop {
        // let dir_now = env::current_dir().unwrap();
        // println!("{}", dir_now.display());
        String::from("shell").arrow_message();
        let user_input = recv_input();
        let user_input_char: Vec<char> = user_input.chars().collect();
        if user_input_char.len() >= 3
            && user_input_char[0] == 'c'
            && user_input_char[1] == 'd'
            && user_input_char[2] == ' '
        {
            let user_input_split: Vec<&str> = user_input.split(" ").collect();
            if user_input_split.len() >= 2 {
                let new_path = Path::new(user_input_split[1]);
                env::set_current_dir(new_path).unwrap();
            }
        } else if (user_input_char.len() == 1 && user_input_char[0] == 'b')
            || (user_input_char.len() == 4
                && user_input_char[0] == 'b'
                && user_input_char[1] == 'a'
                && user_input_char[2] == 'c'
                && user_input_char[3] == 'k')
        {
            String::from("exit shell mode...").info_message();
            break;
        }

        let output = if cfg!(target_os = "windows") {
            match Command::new("pwsh").arg("/C").arg(user_input).output() {
                Ok(o) => o,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        } else if cfg!(target_os = "linux") {
            match Command::new("sh").arg("-c").arg(user_input).output() {
                Ok(o) => o,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        } else {
            let err = String::from("unsupport os");
            err.error_message();
            panic!("{}", err);
        };
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }
}

fn main() {
    // run backend first
    let args = Args::parse();
    // let debug = args.debug;

    let log = args.log;
    LOG_FLAG.set(log).unwrap();
    let verbose = args.verbose;
    VERBOSE_FLAG.set(verbose).unwrap();

    ctrlc::set_handler(move || {
        "bye~".to_string().info_message();
        std::process::exit(0);
    })
    .expect("set ctrlc failed");

    let proxy: Option<String> = match args.proxy.as_str() {
        NULL_VALUE => None,
        _ => Some(args.proxy.to_string()),
    };
    println!("{}\n{}", WELCOME_INFO.bold().red(), VERSION.bold().green());
    // Parameters
    let mut p = Parameters::new();
    // p.add_bool("debug", debug);
    p.add_str("proxy", proxy);
    // Commands
    let mut commands = Commands::new("main", 0);
    commands.add("search", "sr", search, false, vec![], vec![], vec![]);
    commands.add("watchdog", "wd", watchdog, false, vec![], vec![], vec![]);
    commands.add("brute", "bt", brute, false, vec![], vec![], vec![]);
    commands.add("sqltools", "st", sqltools, false, vec![], vec![], vec![]);
    commands.add("honeypot", "hp", honeypot, false, vec![], vec![], vec![]);
    commands.add("shell", "sh", shell, false, vec![], vec![], vec![]);
    commands.run(&mut p);
}
