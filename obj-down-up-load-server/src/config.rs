use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub data_dir: String,
    pub auth_token: String,
}
static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    #[cfg(debug_assertions)]
    fn path() -> &'static str {
        "config.server.toml"
    }

    #[cfg(not(debug_assertions))]
    fn path() -> &'static str {
        "/etc/obj-down-up-load/config.toml"
    }

    pub fn init() {
        println!("Reading config from {}...", Self::path());
        let config =
            toml::from_str(&std::fs::read_to_string(Self::path()).expect("Unable to read config"))
                .expect("Invalid TOML config");
        println!("Config is {:#?}", config);
        CONFIG.set(config).unwrap();
    }

    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }
}
