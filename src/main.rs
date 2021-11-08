use atty::Stream;
use colored::*;
use serde_json::Value;
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "a", long = "label", default_value = "label")]
    label: String,
    #[structopt(short = "m", long = "message", default_value = "message")]
    message: String,
    #[structopt(short = "l", long = "level", default_value = "level")]
    level: String,
    // #[structopt(short = "f", long = "filter", default_value = vec![])]
    // filter: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    if atty::is(Stream::Stdin) {
        //println!("I'm not");
    } else {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().flatten().collect();
        let together = lines.join("\n");

        match serde_json::from_str::<Value>(&together) {
            Result::Ok(v) => {
                println!(
                    "[{}] {}",
                    v[opt.level].as_str().unwrap().blue(),
                    v[opt.message].as_str().unwrap_or("")
                );
            }
            _ => {
                // do nothing
            }
        }
    }
}
