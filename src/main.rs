use structopt::StructOpt;

use hyper::rt;
use hyper_tls::HttpsConnector;
use hyper::Body;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper::rt::Future;
use hyper::rt::Stream;
use hyper::http::Uri;

use regex::Regex;
use std::str;

static INSTAGRAM_URL: &str = "https://www.instagram.com/";
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

fn csrf_token(client: &Client<HttpsConnector<HttpConnector>, Body>) -> impl Future<Item=(), Error=()> {
    client
        .get(Uri::from_static(INSTAGRAM_URL))
        .map(|res| {
            println!("Response: {}", res.status());

            let entire_body =
                res
                    .into_body()
                    .concat2()
                    .and_then(|c| {
                        str::from_utf8(&c).map(str::to_owned).map_err(|x| panic!("Not a string: {}", x))
                    })
                    .wait();

            let csrf_re: Regex = Regex::new(CSRF_RE).unwrap();
            let b = entire_body.ok().unwrap();
            let token: String = csrf_re.captures_iter(&b).map(|cap| cap.get(1).unwrap().as_str()).collect();

            println!("{}", token);
            ()
        })
        .map_err(|err| {
            println!("Error: {}", err)
        })
}

fn login(client: &Client<HttpsConnector<HttpConnector>, Body>, login: &str, pwd: &str) -> impl Future<Item=(), Error=()> {
    println!("Trying to login into instagram account '{}'", login);

    csrf_token(client)
}

fn promote_me(opts: &Opts) -> impl Future<Item=(), Error=()> {
    let https = hyper_tls::HttpsConnector::new(16).unwrap();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let acc = &opts.account;
    let pwd = &opts.password;

    login(&client, &acc, &pwd)
}

fn main() {
    let opts = Opts::from_args();
    println!("Using command line parameters: {:?}", opts);

    rt::run(promote_me(& opts));
}
