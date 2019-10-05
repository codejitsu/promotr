mod config;
mod instagram_promotr;

extern crate hyper;

use hyper::rt;
use std::collections::HashMap;

fn parse_param(param: &str) -> Option<(&str, &str)> {
    match param.find('=') {
        Some(index) => Some(param.split_at(index)).map(|p| (p.0, &p.1[1..])),
        None => None
    }
}

fn main() {
    println!("*promotr* make your insta great again!");
    let mut app_config = config::Config::load();

    let mut params = HashMap::<String, String>::new();

    for arg in std::env::args().skip(1) {
        let arg_str = String::from(arg);
        let parsed = parse_param(&arg_str);

        match parsed {
            Some(p) => {
                params.insert(String::from(p.0), String::from(p.1));
                ()
            },
            None => ()
        }
    }

    if params.contains_key("account") {
        app_config.update_account(params.get("account").unwrap());
    }

    if params.contains_key("password") {
        app_config.update_password(params.get("password").unwrap());
    }

    println!("Using config: {}", app_config);

    rt::run(instagram_promotr::InstaPromoter::promote_me(&mut app_config));
}
