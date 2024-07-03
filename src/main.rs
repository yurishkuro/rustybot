use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about = "rustybot", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
    if let Ok(workspace) = env::var("GITHUB_WORKSPACE") {
        println!("GITHUB_WORKSPACE: {}", workspace);
    } else {
        println!("GITHUB_WORKSPACE is not set");
    }
}
