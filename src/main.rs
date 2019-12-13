extern crate dotenv;

use dotenv::dotenv;
use kinderdom::Config;
use std::env;
use std::process;

fn main() {
    dotenv().ok();

    let config = Config::new(env::vars()).unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        process::exit(1);
    });

    if let Err(e) = kinderdom::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
