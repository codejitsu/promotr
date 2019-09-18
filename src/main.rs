mod config;

fn main() {
    println!("*promotr* make your insta great again!");
    let app_config = config::Config::load();

    println!("Using config: {}", app_config);
}
