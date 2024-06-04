use clap::Parser;

mod scan;
mod utils;
mod errors;

const WELCOME_INFO: &str = r"
  _    _            _              ____            __   __
 | |  | |          | |            / __ \           \ \ / /
 | |__| | __ _  ___| | _____ _ __| |  | |_ __   ___ \ V / 
 |  __  |/ _` |/ __| |/ / _ \ '__| |  | | '_ \ / _ \ > <  
 | |  | | (_| | (__|   <  __/ |  | |__| | | | |  __// . \ 
 |_|  |_|\__,_|\___|_|\_\___|_|   \____/|_| |_|\___/_/ \_\";

/// HackerOneX that integrates most of the tools you need
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// SYN port scanning
    #[arg(long = "sps", action)]
    sysportscan: bool,
    /// Threads number
    #[arg(short, long, default_value_t = 8)]
    threadsnum: usize,
    /// The timeout for all network work
    #[arg(short, long, default_value_t = 1.0)]
    timeout: f32,
    /// Log to file
    #[arg(short, long, action)]
    log: bool,
    /// Set in verbose mode
    #[arg(short, long, action)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let version = env!("CARGO_PKG_VERSION");
    let welcome_info = format!("{} v{}\n", WELCOME_INFO, version);
    println!("{}", welcome_info);
    if args.sysportscan {}
}
