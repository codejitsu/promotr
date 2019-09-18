extern crate config;

use std::fmt;

pub struct Config {
    account: String,
    hashtags: Vec<String>
}

impl Config {
    pub fn load() -> Config {
        let mut settings = config::Config::default();
        settings
            // Add in `./Settings.toml`
            .merge(config::File::with_name("Settings")).unwrap()
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(config::Environment::with_prefix("APP")).unwrap();

        Config {
            account: settings.get("account").ok().unwrap_or(String::from("testaccount")),
            hashtags: settings.get("hashtags").ok().unwrap_or(Vec::new())
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ account: '{}', hashtags: '{}' ]", self.account, self.hashtags.join(", "))
    }
}
