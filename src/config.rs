use std::env;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let secret = env::var("SECRET")?;
        Ok(Config { secret })
    }
}
