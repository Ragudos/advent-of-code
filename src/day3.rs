

use std::collections::HashMap;

use crate::utils::read_file;

const DIRECTIONS: [[i8; 2]; 8] = [
    [1, 1],
    [1, 0],
    [1, -1], // upper right, up, upper left
    [0, 1],
    [0, -1], // right, left
    [-1, 1],
    [-1, 0],
    [-1, -1], // lower right, down, lower left
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
    let binding = read_file("day3input.txt");
    let line_inputs = binding
        .split('\n')
        .map(|line| line.trim().split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum_of_gear_parts = 0;

    for y in 0..(line_inputs.len() - 1) {
        let row = &line_inputs.get(y).unwrap();

        for x in 0..(row.len() - 1) {
            let char = row.get(x).unwrap();
            let char = char.chars().next().unwrap_or(' ');
            let should_check_if_gear_part = is_gear_part(&char);

            if !should_check_if_gear_part {
                continue;
            }

            let mut visited_indeces: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut adjacent_numbers: Vec<usize> = Vec::new();

            for direction_options in DIRECTIONS {
                let dy = <i8 as Into<isize>>::into(direction_options[0]);
                let dx = <i8 as Into<isize>>::into(direction_options[1]);

                // Continuing means the vertical or horizontal index is negative.
                let Some(vertical_idx) = y.checked_add_signed(dy) else { continue; };
                let Some(horizontal_idx) = x.checked_add_signed(dx) else { continue; };

                if vertical_idx >= line_inputs.len() {
                    continue;
                }

                let Some(row) = line_inputs.get(vertical_idx) else { continue; };
                let character = row.get(horizontal_idx).unwrap();
                let character = character.chars().next().unwrap();

                if !char::is_numeric(character) {
                    continue;
                }

                let mut string_number = String::new();

                string_number.push(character);
                
                let mut row_idx = horizontal_idx;
                let mut direction = -1;
                let mut directions_walked = [false, false];

                if let Some(visited_indeces) = visited_indeces.get_mut(&vertical_idx) {
                    if visited_indeces.contains(&horizontal_idx) {
                        continue;
                    }
                } else {
                    visited_indeces.insert(vertical_idx, vec![horizontal_idx]);
                }

                loop {
                    if directions_walked[0] && directions_walked[1] {
                        break;
                    }

                    if row_idx >=row.len() {
                        break;
                    }

                    let character = row.get(row_idx).unwrap();
                    let character = character.chars().next().unwrap_or(' ');

                    // println!("\tCharacter: {}, Row index: {}, Vertical Index: {}, Visited indeces: {:?}, Current Adjacent Numbers: {:?}, Visited Directions: {:?}, Direction: {:?}", character, row_idx, vertical_idx, visited_indeces, adjacent_numbers, directions_walked, direction_options);

                    let Some(visited_indeces) = visited_indeces.get_mut(&vertical_idx) else {
                        println!("This should not happen.");
                        break;
                    };

                    if visited_indeces.contains(&row_idx) {
                        let Some(new_idx) = row_idx.checked_add_signed(direction) else {
                            // if this is ran, then that means the direction was
                            // -1.
                            directions_walked[0] = true;
                            direction = 1;
                            row_idx += direction as usize;
                            continue;
                        };

                        if row_idx < horizontal_idx {
                            directions_walked[0] = true;
                            direction = 1;
                            row_idx += 1;
                            continue;
                        } else if row_idx > horizontal_idx {
                            directions_walked[1] = true;
                            direction = -1;
                            row_idx = new_idx;
                            continue;
                        }
                    } else {
                        visited_indeces.push(row_idx);
                    }
                    
                    if char::is_numeric(character) {
                        if row_idx < horizontal_idx {
                            string_number.insert(0, character);
                        } else if row_idx > horizontal_idx {
                            string_number.push(character);
                        }
                    } else {
                        if row_idx < horizontal_idx {
                            directions_walked[0] = true;
                            direction = 1;
                        } else if row_idx > horizontal_idx {
                            directions_walked[1] = true;
                            direction = -1;
                        }
                    }

                    let new_idx = row_idx.checked_add_signed(direction);

                    match new_idx {
                        None => {
                            directions_walked[0] = true;
                            direction = 1;
                            row_idx += direction as usize;
                        },
                        Some(new_idx) => row_idx = new_idx,
                    }
                }

                let parsed_number = string_number.parse::<usize>();

                match parsed_number {
                    Ok(parsed_number) => adjacent_numbers.push(parsed_number),
                    Err(_err) => { continue },
                }
            }

            if adjacent_numbers.len() != 2 {
                continue;
            }

            let product = adjacent_numbers[0] * adjacent_numbers[1];

            println!("\tProduct: {}", product);
            println!("Adjacent Numbers: {:?}", adjacent_numbers);

            sum_of_gear_parts += product;
        }

    }

    println!("Sum: {}", sum_of_gear_parts);
}

fn part1() {
    let binding = read_file("day3input.txt");

    let line_inputs = binding
        .split('\n')
        .map(|line| line.trim().split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    // The last item is the empty string, so we don't want to include that.
    for y in 0..(line_inputs.len() - 1) {
        let row = &line_inputs[y];
        let mut is_number = false;
        let mut is_checking = true;
        let mut current_number = String::new();

        println!("Current row idx: {}, Current row: {:?}", y, line_inputs[y]);

        for x in 0..row.len() {
            if let Some(character) =
                get_character(&line_inputs, &y, &x, &SINGLE_POINT)
            {
                if let Some(character) = character.chars().next() {
                    is_number = char::is_numeric(character);
                }
            }

            // If we are not on a number char and we are not checking for an engine part,
            // that means we found an engine part before, so we add.
            if !is_number && !is_checking {
                println!("Current number: {}", current_number);
                sum += current_number.parse::<usize>().unwrap_or(0);
            }

            // If we are not on a number char, then we reset the current number string,
            // since if the current number string was an engine part, it would've been added above.
            // Then, reset the is_checking variable since we are looking for an engine part again.
            if !is_number {
                current_number.clear();
                is_checking = true;
            }

            // If we are checking for an engine part in this row and we are on a number char,
            // check if this number char is an engine part. If so, then we are not checking for an
            // engine part anymore.
            if is_number && is_checking && check_if_engine_part(&line_inputs, &y, &x) {
                is_checking = false;
            }

            if is_number {
                if let Some(character) =
                    get_character(&line_inputs, &y, &x, &SINGLE_POINT)
                {
                    current_number.push_str(character);
                }
            }
        }

        // Edge case for when a number is at the end of a row, and it's an engine part.
        if is_number && !is_checking {
            let number = current_number.parse::<usize>();

            match number {
                Err(_err) => {}
                Ok(number) => sum += number,
            }
        }
    }

    println!("Sum: {}", sum);
}

fn check_if_engine_part<'a>(
    contents: &'a [Vec<&str>],
    idx: &'a usize,
    second_idx: &'a usize,
) -> bool {
    for direction in DIRECTIONS {
        if let Some(character) = get_character(
            contents,
            idx,
            second_idx,
            &(&direction[0], &direction[1]),
        ) {
            if let Some(character) = character.chars().next() {
                let is_dot = is_dot(&character);
                let is_number = char::is_numeric(character);

                if !is_dot && !is_number {
                    return true;
                }
            }
        }
    }

    false
}

fn get_character<'a>(
    contents: &'a [Vec<&str>],
    idx: &'a usize,
    second_idx: &'a usize,
    (y, x): &'a (&i8, &i8),
) -> Option<&'a str> {
    let vertical_direction = idx
        .checked_add_signed(<i8 as Into<isize>>::into(**y))
        .unwrap_or(0);
    let horizontal_direction = second_idx
        .checked_add_signed(<i8 as Into<isize>>::into(**x))
        .unwrap_or(0);

    // println!("\t\tRow idx: {}, Column idx: {}", vertical_direction, horizontal_direction);

    if vertical_direction == contents.len() - 1 {
        return None;
    }

    if horizontal_direction == contents[vertical_direction].len() {
        return None;
    }

    if let Some(row) = contents.get(vertical_direction) {
        return row.get(horizontal_direction).copied();
    }

    None
}

fn is_dot(char: &char) -> bool {
    char == &'.'
}

fn is_gear_part(char: &char) -> bool {
    char == &'*'
}
