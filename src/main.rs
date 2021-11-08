use atty::Stream;
use colored::*;
use serde_json::Value;
use std::io::{self, BufRead};

fn main() {
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
                    v["level"].as_str().unwrap().blue(),
                    v["message"].as_str().unwrap()
                );
            }
            _ => {
                // do nothing
            }
        }
    }
}
