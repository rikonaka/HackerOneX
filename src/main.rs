use clap::Parser;

/// Simple program to search vulnerability fast
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the tool
    #[arg(short, long, default_value = "search")]
    tool: String,

    /// Path of filestag watch
    #[arg(short, long, default_value = "test")]
    filestag: String,

    /// Delay of filestag
    #[arg(short, long, default_value_t = 1.0)]
    delay: f32,

    /// Name of the vulnerability
    #[arg(short, long, default_value = "discuz")]
    name: String,
}

mod search;
mod watchdog;

fn main() {
    let args = Args::parse();
    match args.tool.as_str() {
        "watchdog" => watchdog::filestag::run(&args.filestag, args.delay),
        _ => search::exploitalert::search(&args.name),
    }
}
