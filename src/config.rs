extern crate config;

use std::fmt;

pub struct Config {
    account: String,
    password: String,
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
            password: settings.get("password").ok().unwrap_or(String::from("mytestpassword")),
            hashtags: settings.get("hashtags").ok().unwrap_or(Vec::new())
        }
    }

    pub fn update_account(&mut self, acc: &str) {
        self.account = String::from(acc);
    }

    pub fn update_password(&mut self, pwd: &str) {
        self.password = String::from(pwd);
    }

    pub fn account(&mut self) -> String {
        self.account.clone()
    }

    pub fn password(&mut self) -> String {
        self.password.clone()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ account: '{}', password: '{}', hashtags: '{}' ]", self.account,
            self.password, self.hashtags.join(", "))
    }
}
