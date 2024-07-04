use clap::Parser;
mod github;
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
    let repo_owner = "yurishkuro";
    let repo_name = "rustybot";
    let token = env::var("GITHUB_TOKEN").unwrap_or_default();
    match github::get_open_issues(repo_owner, repo_name, &token) {
        Ok(issues) => {
            for issue in issues {
                println!(
                    "#{} - {} - by {}",
                    issue.number, issue.title, issue.user.login
                );
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
