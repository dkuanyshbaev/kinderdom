use std::env;
use std::error::Error;

pub struct Config {
    pub db: String,
    pub secret: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let db = env::var("DB")?;
        let secret = env::var("SECRET")?;
        Ok(Config { db, secret })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}, {}", config.db, config.secret);
    println!("ok");
    Ok(())
}
