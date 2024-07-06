use clap::Parser;
use std::env;

mod config;
mod config_loader;
mod github;
mod github_client;

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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
    let token = env::var("GITHUB_TOKEN").unwrap_or_default();
    let api_url =
        env::var("GITHUB_API_URL").unwrap_or_else(|_| String::from("https://api.github.com"));
    let gh_client = github_client::Client {
        api_url,
        token,
        repo_owner: String::from("yurishkuro"),
        repo_name: String::from("rustybot"),
    };
    use github::GitHub;
    match gh_client.get_open_issues().await {
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
    config_loader::load_config("issue-manager.yml").expect("Expecting valid config");
}
