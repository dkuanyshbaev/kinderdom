use std::env;
use std::error::Error;

pub struct Config {
    pub db: String,
    pub secret: String,
}

impl Config {
    // pub fn new(mut vars: std::env::Vars) -> Result<Config, &'static str> {
    pub fn new(vars: std::env::Vars) -> Result<Config, std::env::VarError> {
        // let db = env::var("DB")?;
        // Result<std::string::String, std::env::VarError>

        let db = match env::var("DB") {
            Ok(db) => db,
            Err(e) => return Err(e),
        };

        let secret = match env::var("SECRET") {
            Ok(secret) => secret,
            Err(e) => return Err(e),
        };

        Ok(Config { db, secret })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}, {}", config.db, config.secret);
    println!("ok");

    Ok(())
}
