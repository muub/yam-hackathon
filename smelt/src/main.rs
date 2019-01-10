extern crate maxminddb;
extern crate regex;
extern crate bigdecimal;

extern crate ndarray;

use std::env;
use std::process;

use smelt;
use smelt::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);

    println!("Processing for Date: {}", config.date);
    println!("Outputing to {}", config.outfile);
    println!("In file {}", config.filename);

    if let Err(e) = smelt::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
