use std::process;
use std::env;

mod day1;
mod day2;
mod utils;

fn main() {
    let arguments: Vec<_> = env::args().collect();
    let day = arguments[1].parse::<String>().unwrap();
    let day_str = day.as_str();

    match day_str {
        "day1" => day1::main(),
        "day2" => day2::main(),
        _ => {
            println!("Invalid day");
            process::exit(1);
        },
    }
}


