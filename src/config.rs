use std::env;

pub struct Config {
    pub secret: String,
    pub yandex_credentials: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let secret = env::var("SECRET")?;
        let yandex_credentials = env::var("YANDEX_CREDENTIALS")?;
        Ok(Config {
            secret,
            yandex_credentials,
        })
    }
}
