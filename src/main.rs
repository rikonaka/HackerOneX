use anyhow::Result;
use clap::Parser;

mod errors;
mod scan;
mod utils;

use crate::scan::ScanMethods;

const WELCOME_INFO: &str = r"
  _    _            _              ____            __   __
 | |  | |          | |            / __ \           \ \ / /
 | |__| | __ _  ___| | _____ _ __| |  | |_ __   ___ \ V / 
 |  __  |/ _` |/ __| |/ / _ \ '__| |  | | '_ \ / _ \ > <  
 | |  | | (_| | (__|   <  __/ |  | |__| | | | |  __// . \ 
 |_|  |_|\__,_|\___|_|\_\___|_|   \____/|_| |_|\___/_/ \_\";

/// HackerOneX that integrates most of the tools you need to hack
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// SYN port scanning
    #[arg(long = "sps", action)]
    sysportscan: bool,
    /// CONNECT port scanning
    #[arg(long = "cps", action)]
    conportscan: bool,
    /// FIN port scanning
    #[arg(long = "fps", action)]
    finportscan: bool,
    /// NULL port scanning
    #[arg(long = "nps", action)]
    nullportscan: bool,
    /// XMAS port scanning
    #[arg(long = "xps", action)]
    xmasportscan: bool,
    /// ACK port scanning
    #[arg(long = "aps", action)]
    ackportscan: bool,
    /// WINDOW port scanning
    #[arg(long = "wps", action)]
    windowportscan: bool,
    /// MAIMON port scanning
    #[arg(long = "mps", action)]
    maiportscan: bool,
    /// UDP port scanning
    #[arg(long = "ups", action)]
    udpportscan: bool,

    /// Remote os detect
    #[arg(long = "rod", action)]
    remoteosdetect: bool,
    /// Remote os detect (IPv6)
    #[arg(long = "rod6", action)]
    remoteosdetect6: bool,
    /// Top k value for remote os detect
    #[arg(long = "topk", default_value_t = 1)]
    topk: usize,

    /// Target addr
    #[arg(long = "ta", default_value = "192.168.1.1")]
    targetaddr: String,
    /// Target port
    #[arg(long = "tp", default_value = "80")]
    targetport: String,
    /// Threads number
    #[arg(long, default_value_t = 8)]
    thread: usize,
    /// The timeout for all network work
    #[arg(long, default_value_t = 1.0)]
    timeout: f32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let version = env!("CARGO_PKG_VERSION");
    let welcome_info = format!("{} v{}\n", WELCOME_INFO, version);
    println!("{}", welcome_info);

    if args.sysportscan
        || args.conportscan
        || args.finportscan
        || args.nullportscan
        || args.xmasportscan
        || args.ackportscan
        || args.windowportscan
        || args.maiportscan
        || args.udpportscan
    {
        // port scan
        let method = if args.sysportscan {
            ScanMethods::SYN
        } else if args.conportscan {
            ScanMethods::CON
        } else if args.finportscan {
            ScanMethods::FIN
        } else if args.nullportscan {
            ScanMethods::NULL
        } else if args.xmasportscan {
            ScanMethods::XMAS
        } else if args.ackportscan {
            ScanMethods::ACK
        } else if args.windowportscan {
            ScanMethods::WIN
        } else if args.maiportscan {
            ScanMethods::MAI
        } else if args.udpportscan {
            ScanMethods::UDP
        } else {
            // can not reach here
            ScanMethods::SYN
        };
        scan::portscan(
            &args.targetaddr,
            &args.targetport,
            args.thread,
            args.timeout,
            method,
        )?;
    } else if args.remoteosdetect || args.remoteosdetect6 {
        scan::osscan(
            &args.targetaddr,
            args.thread,
            args.timeout,
            args.topk,
        )?;
    }

    Ok(())
}
