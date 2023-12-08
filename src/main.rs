
use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let mut day_str: &str = "";
    let mut part: &str = "";

    for (idx, argument) in arguments.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let splitter = "=";
        let key_value_pair = argument.split(splitter).collect::<Vec<_>>();

        if key_value_pair.len() != 2 {
            println!("Invalid argument: {}", argument);
            process::exit(1);
        }

        let key = key_value_pair[0].trim();
        let value = key_value_pair[1].trim();

        match key {
            "day" => day_str = value,
            "part" => part = value,
            "backtrace" => {
                if value == "true" {
                    std::env::set_var("RUST_BACKTRACE", "1");
                }
            }
            _ => {}
        }
    }

    match day_str {
        "1" => day1::main(),
        "2" => day2::main(part),
        "3" => day3::main(part),
        _ => {
            println!("Invalid day");
            process::exit(1);
        }
    }
}
