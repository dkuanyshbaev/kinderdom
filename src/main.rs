use kinderdom::Config;
use std::process;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        process::exit(1);
    });

    if let Err(e) = kinderdom::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
