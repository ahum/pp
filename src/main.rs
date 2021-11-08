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
    #[structopt(short = "fl", long = "filter_level", default_value = "silly")]
    filter_level: String,
    // #[structopt(short = "f", long = "filter", default_value = vec![])]
    // filter: Vec<String>,
}

const LEVELS: [&str; 7] = ["silly", "debug", "verbose", "info", "warn", "error", "off"];

fn allow_level(requested: &str, limit: &str) -> bool {
    let requested_index = LEVELS.iter().position(|&x| x == requested);
    let limit_index = LEVELS.iter().position(|&x| x == limit);

    println!("requested {}, limit: {}", requested, limit);
    match (requested_index, limit_index) {
        (Some(r), Some(li)) => r >= li,
        _ => false,
    }
}

fn main() {
    let opt = Opt::from_args();
    let opt_level = opt.level.as_str();
    // println!("{:?}", opt);
    if !atty::is(Stream::Stdin) {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().flatten().collect();
        let together = lines.join("\n");

        match serde_json::from_str::<Value>(&together) {
            Result::Ok(v) => {
                let level = v[opt_level].as_str().unwrap();
                if allow_level(level, opt_level) {
                    println!(
                        "[{}] {}",
                        level.blue(),
                        v[opt.message].as_str().unwrap_or("")
                    );
                }
            }
            _ => {
                // do nothing
            }
        }
    }
}
