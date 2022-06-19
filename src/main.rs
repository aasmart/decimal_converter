use std::process;
use binary_decimal::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Encountered an issue parsing arguments: {}", err);
        process::exit(1);
    });

    binary_decimal::run(&config).unwrap_or_else(|err| {
        eprintln!("Encountered an application error: {}", err);
        process::exit(1);
    });
}
