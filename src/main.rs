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

fn allow_level(requested: &str, limit: &String) -> bool {
    let requested_index = LEVELS.iter().position(|&x| x == requested);
    let limit_index = LEVELS.iter().position(|&x| x == limit);
    match (requested_index, limit_index) {
        (Some(r), Some(li)) => r >= li,
        _ => false,
    }
}

// fn main() {

//   let mut input = String::new();

//   loop {
//       match io::stdin().read_line(&mut input) {
//           Ok(n) => {
//               println!("{} bytes read", n);
//               println!("{}", input);
//           }
//           Err(error) => println!("error: {}", error),
//       }
//   }

//     // let opt_level = opt.filter_level;
//     // // let opt_level = opt.level.as_str();
//     // if !atty::is(Stream::Stdin) {
//     //     let stdin = io::stdin();
//     //     let lines: Vec<String> = stdin.lock().lines().flatten().collect();
//     //     let together = lines.join("\n");

//     //     println!("together: {:?}", together);

//     //     match serde_json::from_str::<Value>(&together) {
//     //         Result::Ok(v) => {
//     //             let level = v[&opt.level].as_str().unwrap();
//     //             if allow_level(level, &opt_level) {
//     //                 println!(
//     //                     "[{}] {}",
//     //                     level.blue(),
//     //                     v[opt.message].as_str().unwrap_or("")
//     //                 );
//     //             }
//     //         }
//     //         _ => {
//     //             // do nothing
//     //         }
//     //     }
//     // } else {
//     //     println!("??? what?")
//     // }
// }

fn get_message(v: &Value) -> String {
    match v {
        Value::Object(m) => serde_json::to_string(v).unwrap(),
        Value::String(s) => s.clone(),
        //v.as_str().unwrap(),
        _ => String::from("NO-MESSAGE"),
    }
}
fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    let opt_level = opt.filter_level;
    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                //print!("{}", line);
                match serde_json::from_str::<Value>(&line) {
                    Result::Ok(v) => {
                        // println!("{}", line);
                        let level = v[&opt.level].as_str().unwrap();
                        let label = v[&opt.label].as_str().unwrap_or("NO-LABEL");

                        let msg = get_message(&v[&opt.message]);

                        if allow_level(level, &opt_level) {
                            println!("[{}:{}] {}", level.blue(), label.red(), msg);
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
