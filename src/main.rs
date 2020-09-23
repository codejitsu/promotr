use structopt::StructOpt;
use reqwest::Error;
use regex::Regex;
use json::object;
use std::{thread, time};
use rand::seq::SliceRandom;

static INSTAGRAM_URL: &str = "https://www.instagram.com/";
static LOGIN_URL: &str = "https://www.instagram.com/accounts/login/ajax/";

static CSRF_RE: &str = r#""csrf_token":"(\w+)""#;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    account: String,

    #[structopt(short, long)]
    password: String,

    #[structopt(short, long)]
    hashtags: Vec<String>
}

fn csrf_token() -> Result<String, Error> {
    let csrf_re = Regex::new(CSRF_RE).unwrap();

    let token_result = reqwest::blocking::get(INSTAGRAM_URL)?
        .text()
        .map(|body| 
            csrf_re.captures_iter(&body)
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect()
        );

    token_result    
}

fn login(token: String, opts: &Opts) -> Result<String, Error> {
    let wait_sec = time::Duration::from_secs(5);
    thread::sleep(wait_sec);

    println!("Using csrf_token: {}", token);

    let user_agents = vec![
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FSL 7.0.6.01001)",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FSL 7.0.7.01001)",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FSL 7.0.5.01003)",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:12.0) Gecko/20100101 Firefox/12.0",
        "Mozilla/5.0 (X11; U; Linux x86_64; de; rv:1.9.2.8) Gecko/20100723 Ubuntu/10.04 (lucid) Firefox/3.6.8",
        "Mozilla/5.0 (Windows NT 5.1; rv:13.0) Gecko/20100101 Firefox/13.0.1",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:11.0) Gecko/20100101 Firefox/11.0",
        "Mozilla/5.0 (X11; U; Linux x86_64; de; rv:1.9.2.8) Gecko/20100723 Ubuntu/10.04 (lucid) Firefox/3.6.8",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; .NET CLR 1.0.3705)",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)",
        "Opera/9.80 (Windows NT 5.1; U; en) Presto/2.10.289 Version/12.01",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 2.0.50727)",
        "Mozilla/5.0 (Windows NT 5.1; rv:5.0.1) Gecko/20100101 Firefox/5.0.1",
        "Mozilla/5.0 (Windows NT 6.1; rv:5.0) Gecko/20100101 Firefox/5.02",
        "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/13.0.782.112 Safari/535.1",
        "Mozilla/4.0 (compatible; MSIE 6.0; MSIE 5.5; Windows NT 5.0) Opera 7.02 Bork-edition [en]",
    ];

    let user_agent = user_agents.choose(&mut rand::thread_rng()).unwrap();

    let client = reqwest::blocking::Client::new();

    let login_data = object! {
        username: format!("{}", opts.account),
        password: format!("{}", opts.password)
    };

    let login_response = client.post(LOGIN_URL)
        .header("X-CSRFToken", token)
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Connection", "keep-alive")
        .header("Host", "www.instagram.com")
        .header("Origin", "https://www.instagram.com")
        .header("Referer", "https://www.instagram.com/")
        .header("User-Agent", user_agent.to_owned())
        .header("X-Instagram-AJAX", "1")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("X-Requested-With", "XMLHttpRequest")
        .body(login_data.dump())
        .send()?
        .text();

    login_response
}

fn run(opts: &Opts) -> Result<(), Error> {
    println!("Using command line parameters: {:?}", opts);

    let login_result = 
        csrf_token()
        .and_then(|token| login(token, opts));

    match login_result {
        Ok(response) => {
            println!("response: {}", response);
            Ok(())
        }
        Err(e) => Err(e)
    }    
}

fn main() {
    let opts = Opts::from_args();
    
    match run(&opts) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}
