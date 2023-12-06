use std::str::Split;

use crate::utils::read_file;

const DIRECTIONS: [[i8; 2]; 8] = [
    [1, 1], [1, 0], [1, -1], // upper right, up, upper left
    [0, 1], [0, -1], // right, left
    [-1, 1], [-1, 0], [-1, -1], // lower right, down, lower left
];

const SINGLE_POINT: (&i8, &i8) = (&0, &0);

pub fn main(part: &str) {
    match part {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid part");
            std::process::exit(1);
        }
    }
}

fn part2() {
    println!("Part 2 not yet implemented");
}

fn part1() {
    let binding = read_file("day3input.txt");

    let line_inputs = binding.split('\n').map(| line | line.trim().split("").collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;

    // The last item is the empty string, so we don't want to include that.
    for y in 0..(line_inputs.len() - 1) {
        let row = &line_inputs[y];
        let mut is_number = false;
        let mut is_checking = true;
        let mut current_number = String::new();

        println!("Current row idx: {}, Current row: {:?}", y, line_inputs[y]);

        for x in 0..row.len() {
            if let Some(character) = get_character(&line_inputs, &y, &x, &SINGLE_POINT) {
                if let Some(character) = character.chars().next() {
                    is_number = char::is_numeric(character);
                }
            }

            if !is_number && !is_checking {
                println!("Current number: {}", current_number);
                sum += current_number.parse::<usize>().unwrap_or(0);
            }

            if !is_number {
                current_number.clear();
                is_checking = true;
            }
            
            if is_number && is_checking {
                for direction in DIRECTIONS {
                    if let Some(character) = get_character(&line_inputs, &y, &x, &(&direction[0], &direction[1])) {
                        if let Some(character) = character.chars().next() {
                            let is_dot = is_dot(&character);
                            let is_number = char::is_numeric(character);

                            // This means that the direction we checked, which is within a +1 radius of the current point, is a symbol.
                            // We are pretty much forming a box within the current character we are
                            // checking, and if any of the characters within that box is a symbol, then
                            // we know that the current character is a part of an engine part number.
                            if !is_dot && !is_number {
                                is_checking = false;
                                break;
                            }

                        }
                    };
                }
            }

            if is_number {
                if let Some(character) = get_character(&line_inputs, &y, &x, &SINGLE_POINT) {
                    current_number.push_str(character);
                }
            }
        }

        if is_number && !is_checking {
            sum += current_number.parse::<usize>().unwrap();
        }
    }

    println!("Sum: {}", sum);
}

fn get_character<'a>(contents: &'a [Vec<&str>], idx: &'a usize, second_idx: &'a usize, (y, x): &'a (&i8, &i8)) -> Option<&'a str> {
    let vertical_direction = idx.checked_add_signed(<i8 as Into<isize>>::into(**y)).unwrap_or(0);
    let horizontal_direction = second_idx.checked_add_signed(<i8 as Into<isize>>::into(**x)).unwrap_or(0);

    println!("\t\tRow idx: {}, Column idx: {}", vertical_direction, horizontal_direction);

    if vertical_direction == contents.len() - 1 {
        return None;
    }

    if horizontal_direction == contents[vertical_direction].len() {
        return None;
    }

    Some(contents[vertical_direction][horizontal_direction])
}

fn is_dot(char: &char) -> bool {
    char == &'.'
}

