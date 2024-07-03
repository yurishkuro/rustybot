use clap::{App, Arg};

fn main() {
    let app = App::new("rustybot")
        .version("1.0")
        .author("Yuri Shkurov")
        .about("rustybot")
        .arg(Arg::with_name("name")
             .long("name")
             .short("n")
             .takes_value(true)
             .default_value("World")
             .help("Sets the name to greet"));
    let matches = app.get_matches();
    let name = matches.value_of("name").unwrap();
    println!("Hello, {}!", name);
}