use clap::Parser;

/// Simple program to search vulnerability fast
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the vulnerability
    #[arg(short, long, default_value = "discuz")]
    name: String,
}

mod search;

fn main()  {
    let args = Args::parse();
    search::exploitalert::search(&args.name);
}