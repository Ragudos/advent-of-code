use std::str::Split;

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
        let row = &line_inputs[y];

        let mut is_a_gear_part = false;
        let mut first_engine_part_number = String::new();
        let mut second_engine_part_number = String::new();

        // println!("Current row idx: {}, Current row: {:?}", y, line_inputs[y]);

        for x in 0..row.len() {
            if let Some(character) =
                get_character(&line_inputs, &y, &x, &SINGLE_POINT)
            {
                if let Some(character) = character.chars().next() {
                    is_a_gear_part = is_gear_part(&character);
                }
            }

            if !is_a_gear_part {
                continue;
            }

            let mut amount_of_adjacents = 0;
            let mut is_gathering_number = false;
            let mut has_gathered_number = false;
            // -1, 0, 1
            let mut vertical_directions_that_matched = [false, false, false];

            println!("y {}, x {}", y, x);

            for direction in DIRECTIONS {
                let direction = (&direction[0], &direction[1]);

                println!("Direction: {:?}", direction);

                if let Some(character) =
                    get_character(&line_inputs, &y, &x, &direction)
                {
                    if let Some(character) = character.chars().next() {
                        if char::is_numeric(character) {
                            println!("\tCharacter: {}, Adjacents: {}, Did find number: {}, Current sum: {}", character, amount_of_adjacents, has_gathered_number, sum_of_gear_parts);

                            if (vertical_directions_that_matched[0]
                                && *direction.0 == -1)
                                || (vertical_directions_that_matched[1]
                                    && *direction.0 == 0)
                                || (vertical_directions_that_matched[2]
                                    && *direction.0 == 1)
                            {
                                has_gathered_number = false;
                                is_gathering_number = false;
                                continue;
                            }

                            if amount_of_adjacents == 0 {
                                if let Some(vertical_direction) = y
                                    .checked_add_signed(
                                        <i8 as Into<isize>>::into(*direction.0),
                                    )
                                {
                                    first_engine_part_number =
                                        get_full_number_in_row(
                                            &line_inputs[vertical_direction],
                                            (x as isize)
                                                .checked_add(
                                                    *direction.1 as isize,
                                                )
                                                .unwrap_or(x as isize),
                                            &-1,
                                            &mut String::new(),
                                            &mut [&false, &false],
                                            &mut Vec::new(),
                                        )
                                        .to_string();
                                    has_gathered_number = true;

                                    if *direction.0 == -1 {
                                        vertical_directions_that_matched[0] =
                                            true;
                                    } else if *direction.0 == 0 {
                                        vertical_directions_that_matched[1] =
                                            true;
                                    } else if *direction.0 == 1 {
                                        vertical_directions_that_matched[2] =
                                            true;
                                    }
                                }
                            }

                            if amount_of_adjacents == 1 {
                                if let Some(vertical_direction) = y
                                    .checked_add_signed(
                                        <i8 as Into<isize>>::into(*direction.0),
                                    )
                                {
                                    println!("\tVertical direction: {}, Horizontal direction: {}", vertical_direction, x);
                                    second_engine_part_number =
                                        get_full_number_in_row(
                                            &line_inputs[vertical_direction],
                                            (x as isize)
                                                .checked_add(
                                                    *direction.1 as isize,
                                                )
                                                .unwrap_or(x as isize),
                                            &-1,
                                            &mut String::new(),
                                            &mut [&false, &false],
                                            &mut Vec::new(),
                                        )
                                        .to_string();
                                    has_gathered_number = true;

                                    if *direction.0 == -1 {
                                        vertical_directions_that_matched[0] =
                                            true;
                                    } else if *direction.0 == 0 {
                                        vertical_directions_that_matched[1] =
                                            true;
                                    } else if *direction.0 == 1 {
                                        vertical_directions_that_matched[2] =
                                            true;
                                    }
                                }
                            }

                            if !is_gathering_number && has_gathered_number {
                                amount_of_adjacents += 1;
                            }

                            is_gathering_number = true;
                            println!("\t\tFirst engine part number: {}, Second engine part number: {}", first_engine_part_number, second_engine_part_number);
                        }
                    }
                }
            }

            // A gear part is adjacent to exactly two number parts.
            // Since a gear part is a symbol, that means the adjacent numbers are already an engine
            // part, so we don't need to check for them.
            if amount_of_adjacents != 2 {
                first_engine_part_number.clear();
                second_engine_part_number.clear();
                continue;
            }

            let first_engine_part =
                first_engine_part_number.parse::<usize>().unwrap_or(0);
            let second_engine_part =
                second_engine_part_number.parse::<usize>().unwrap_or(0);

            sum_of_gear_parts += first_engine_part * second_engine_part;

            first_engine_part_number.clear();
            second_engine_part_number.clear();
        }
    }

    println!("Sum: {}", sum_of_gear_parts);
}

fn get_full_number_in_row<'a>(
    row: &'a Vec<&str>,
    idx_of_found_number_char: isize,
    current_horizontal_direction: &'a i8,
    current_number: &'a mut String,
    visited: &'a mut [&bool; 2],
    visited_indeces: &'a mut Vec<usize>,
) -> &'a str {
    // println!("Index: {}, Row: {:?}, Current direction: {}, Current number: {}, Visited: {:?}", idx_of_found_number_char, row, current_horizontal_direction, current_number, visited);

    // This means that if we already wen to the left and right of the row string, then we are done.
    if *visited[0] && *visited[1] {
        return current_number;
    }

    if let Some(idx) =
        idx_of_found_number_char.checked_add(<i8 as Into<isize>>::into(
            *current_horizontal_direction,
        ))
    {
        if idx < 0 {
            visited[0] = &true;

            return get_full_number_in_row(
                row,
                idx_of_found_number_char,
                &1,
                current_number,
                visited,
                visited_indeces,
            );
        }

        if (idx as usize) > row.len() {
            visited[1] = &true;

            return get_full_number_in_row(
                row,
                idx,
                &-1,
                current_number,
                visited,
                visited_indeces,
            );
        }

        if visited_indeces.contains(&(idx as usize)) {
            return get_full_number_in_row(
                row,
                idx,
                current_horizontal_direction,
                current_number,
                visited,
                visited_indeces,
            );
        }

        let character = row[idx as usize];

        visited_indeces.push(idx as usize);

        if idx < idx_of_found_number_char {
            if let Some(character) = character.chars().next() {
                if char::is_numeric(character) {
                    current_number.insert_str(0, &character.to_string());

                    return get_full_number_in_row(
                        row,
                        idx,
                        current_horizontal_direction,
                        current_number,
                        visited,
                        visited_indeces,
                    );
                } else {
                    visited[0] = &true;

                    return get_full_number_in_row(
                        row,
                        idx,
                        &1,
                        current_number,
                        visited,
                        visited_indeces,
                    );
                }
            } else {
                visited[0] = &true;

                return get_full_number_in_row(
                    row,
                    idx,
                    &1,
                    current_number,
                    visited,
                    visited_indeces,
                );
            }
        } else if idx > idx_of_found_number_char {
            if let Some(character) = character.chars().next() {
                if char::is_numeric(character) {
                    current_number.push_str(&character.to_string());

                    return get_full_number_in_row(
                        row,
                        idx,
                        current_horizontal_direction,
                        current_number,
                        visited,
                        visited_indeces,
                    );
                } else {
                    visited[1] = &true;

                    return get_full_number_in_row(
                        row,
                        idx,
                        &-1,
                        current_number,
                        visited,
                        visited_indeces,
                    );
                }
            } else {
                visited[1] = &true;

                return get_full_number_in_row(
                    row,
                    idx,
                    &-1,
                    current_number,
                    visited,
                    visited_indeces,
                );
            }
        } else {
            return get_full_number_in_row(
                row,
                idx,
                current_horizontal_direction,
                current_number,
                visited,
                visited_indeces,
            );
        }
    };

    current_number
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
            if is_number && is_checking {
                if check_if_engine_part(&line_inputs, &y, &x) {
                    is_checking = false;
                }
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

    Some(contents[vertical_direction][horizontal_direction])
}

fn is_dot(char: &char) -> bool {
    char == &'.'
}

fn is_gear_part(char: &char) -> bool {
    char == &'*'
}
