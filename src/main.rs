use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem when parsing arguments: {err}");
        process::exit(1);
    });

    minigrep::run(config);
}

