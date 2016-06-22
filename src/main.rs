#[macro_use]
extern crate log;
extern crate env_logger;

extern crate ctrlc;
extern crate clap;
extern crate gooddata_fs;
extern crate users;

use clap::{Arg, App};

use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gooddata_fs::*;

const DESCRIPTION: &'static str = "GoodData as Filesystem"; // env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Received ctrl+c, exiting");
        r.store(false, Ordering::SeqCst);
        exit(0);
    });

    // Specify program options
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author("Tomas Korcak <korczis@gmail.com>")
        .arg(Arg::with_name("server")
            .help("Server to use")
            .takes_value(true)
            .short("s")
            .long("server")
            .default_value(rest::url::SERVER))
        .arg(Arg::with_name("token")
            .help("Token for creating of projects")
            .takes_value(true)
            .short("t")
            .long("token"))
        .arg(Arg::with_name("environment")
            .help("Environment for creating of projects")
            .takes_value(true)
            .short("e")
            .long("environment")
            .default_value("TESTING"))
        .arg(Arg::with_name("driver")
            .help("Driver for creating of projects")
            .takes_value(true)
            .short("d")
            .long("driver")
            .default_value("Pg"))
        .arg(Arg::with_name("cache-size")
            .help("LRU Cache Size")
            .takes_value(true)
            .short("c")
            .long("cache-size")
            .default_value("32768"))
        .arg(Arg::with_name("username")
            .help("GoodData Username")
            .use_delimiter(false)
            .required(true)
            .index(1))
        .arg(Arg::with_name("password")
            .help("GoodData Password")
            .use_delimiter(false)
            .required(true)
            .index(2))
        .arg(Arg::with_name("mountpoint")
            .help("Mount Point")
            .required(true)
            .index(3))
        .get_matches();

    env_logger::init().unwrap();

    // Parse required program options
    let username = matches.value_of("username").unwrap().to_string();
    let password = matches.value_of("password").unwrap().to_string();
    let mountpoint = matches.value_of("mountpoint").unwrap().to_string();
    let server = matches.value_of("server").unwrap().to_string();
    let token = matches.value_of("token").map(|value| value.to_string());
    let environment = matches.value_of("environment").map(|value| value.to_string());
    let driver = matches.value_of("driver").map(|value| value.to_string());
    let cache_size = matches.value_of("cache-size").map(|value| value.to_string());

    // Create instance of GoodData HTTP Connector
    let connector = gooddata_fs::gd::Connector::new(server,
                                                    cache_size.unwrap().parse::<usize>().unwrap());
    // Create instance of GoodData REST API Client
    let mut gd = gooddata_fs::gd::GoodDataClient::new(connector, token, environment, driver);
    gd.connect(username, password);

    // Create GoodData Filesystem instance
    let fs = fs::GoodDataFS {
        client: gd,
        users_cache: users::UsersCache::new(),
    };

    // Mount GoodData Filesystem
    fs.mount(mountpoint);

}
