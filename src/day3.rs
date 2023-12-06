
use crate::utils::read_file;

pub fn main(part: &str) {
    match part {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid part");
            std::process::exit(1);
        },
    }
}

fn part2() {
    println!("Part 2 not yet implemented");
}

fn part1() {
    let binding = read_file("day3test.txt");
}

