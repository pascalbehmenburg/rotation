use std::path::PathBuf;

pub struct Config {
    pub port: u16,
    pub key: PathBuf,
    pub cert: PathBuf,
}

impl Config {
    pub fn new(port: u16, key: PathBuf, cert: PathBuf) -> Self {
        Config { port, key, cert }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8080,
            key: PathBuf::from("certs/key.pem"),
            cert: PathBuf::from("certs/cert.pem"),
        }
    }
}
