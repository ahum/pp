use colored::*;
use serde_json::Value;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::TryInto,
    hash::{Hash, Hasher},
    io::{self},
};
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
    // TODO: #[structopt(short = "f", long = "filter", default_value = vec![])]
    // filter: Vec<String>,
    // TODO:
    // format: Vec<String>,
    // TODO:
    // passthrough: boolean = false,
}

const LEVELS: [&str; 7] = ["silly", "debug", "verbose", "info", "warn", "error", "off"];

fn allow_level(requested: &str, limit: &String) -> bool {
    let requested_index = LEVELS.iter().position(|&x| x == requested);
    let limit_index = LEVELS.iter().position(|&x| x == limit);
    match (requested_index, limit_index) {
        (Some(r), Some(li)) => r >= li,
        _ => false,
    }
}

fn get_message(v: &Value) -> String {
    match v {
        Value::Object(m) => serde_json::to_string(v).unwrap(),
        Value::String(s) => s.clone(),
        //v.as_str().unwrap(),
        _ => String::from("NO-MESSAGE"),
    }
}

fn colorize_level(level: &str) -> ColoredString {
    match level {
        "debug" => level.blue(),
        "warn" => level.yellow(),
        "error" => level.red(),
        "silly" => level.purple(),
        "info" => level.white(),
        "verbose" => level.green(),
        _ => level.white(),
    }
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fn colorize_label(label: &str, label_to_color: &mut HashMap<String, Rgb>) -> ColoredString {
    let c: &mut Rgb = label_to_color
        .entry(String::from(label))
        .or_insert_with(|| {
            let mut hasher = DefaultHasher::new();
            label.hash(&mut hasher);
            let done = hasher.finish();
            let r: u8 = ((done & 0xFF0000) >> 16).try_into().unwrap();
            let g: u8 = ((done & 0x00FF00) >> 8).try_into().unwrap();
            let b: u8 = (done & 0x0000FF).try_into().unwrap();
            Rgb { r, g, b }
        });

    label.color(Color::TrueColor {
        r: c.r,
        g: c.g,
        b: c.b,
    })
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let mut label_to_color: HashMap<String, Rgb> = HashMap::new();

    let opt_level = opt.filter_level;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                match serde_json::from_str::<Value>(&line) {
                    Result::Ok(v) => {
                        let level = v[&opt.level].as_str().unwrap();
                        let label = v[&opt.label].as_str().unwrap_or("NO-LABEL");

                        if allow_level(level, &opt_level) {
                            let msg = get_message(&v[&opt.message]);
                            println!(
                                "[{}] {} | {}",
                                colorize_level(level),
                                colorize_label(label, &mut label_to_color),
                                msg
                            );
                        }
                    }
                    _ => {
                        // do nothing
                        //print!("cant parse as json")
                    }
                }
            }
        }
    }

    Ok(())
}
