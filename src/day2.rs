use std::process;

use crate::utils::{read_file, self};

const POSSIBLE_VALUES_RGB: [u8; 3] = [12, 13, 14];

pub fn main(what_to_run: &str) {
    if what_to_run == "1" {
        part_one();
    } else if what_to_run == "2" {
        part_two();
    } else {
        println!("Part specified does not exist. Either 1 or 2 should be the second argument");
        process::exit(1);
    }
}

fn part_two() {
    let binding = read_file("day2input.txt");
    let input_lines = binding.split("\r\n").into_iter();

    let mut tmp_arr_listofvalues_rgb: Vec<Vec<usize>> = utils::create_two_dimensional_array(3, 3, 0);

    let mut sum_of_all_powers = 0;

    for input in input_lines {
        let Some((game, game_set)) = input.split_once(":") else { continue };
        let game_set_results = game_set.split(";");

        for (set_result_idx, set_result) in game_set_results.enumerate() {
            let set_results = &set_result.trim();
            let results = set_results.split(",");

            println!("\n-- {} --\n", game);

            for result in results {
                let result = &result.trim();

                let mut num_color_split = result.split(" ");
                let amount = num_color_split.next().unwrap();
                let color = num_color_split.next().unwrap();

                let amount = amount.parse::<usize>().unwrap();

                println!("Amount: {}, Color: {}", amount, color);

                match color {
                    "red" => {
                        if tmp_arr_listofvalues_rgb[0].len() - 1 == set_result_idx {
                            tmp_arr_listofvalues_rgb[0].push(amount);
                        } else {
                            tmp_arr_listofvalues_rgb[0][set_result_idx] = amount;
                        }
                    },
                    "green" => {
                        if tmp_arr_listofvalues_rgb[1].len() - 1 == set_result_idx {
                            tmp_arr_listofvalues_rgb[1].push(amount);
                        } else {
                            tmp_arr_listofvalues_rgb[1][set_result_idx] = amount;
                        }
                    },
                    "blue" => {
                        if tmp_arr_listofvalues_rgb[2].len() - 1 == set_result_idx {
                            tmp_arr_listofvalues_rgb[2].push(amount);
                        } else {
                            tmp_arr_listofvalues_rgb[2][set_result_idx] = amount;
                        }
                    },
                    _ => {}
                }
            }

            println!("\tList of values: {:?}", tmp_arr_listofvalues_rgb);
        }

        let max_in_red = tmp_arr_listofvalues_rgb[0].iter().max().unwrap_or(&0);
        let max_in_green = tmp_arr_listofvalues_rgb[1].iter().max().unwrap_or(&0);
        let max_in_blue = tmp_arr_listofvalues_rgb[2].iter().max().unwrap_or(&0);

        println!("\t\tMax in red: {:?}, Max in green: {:?}, Max in blue: {:?}", max_in_red, max_in_green, max_in_blue);

        let power_of_three_balls = max_in_red * max_in_green * max_in_blue;
    
        sum_of_all_powers += power_of_three_balls;
        
        utils::reset_two_dimensional_array(&mut tmp_arr_listofvalues_rgb, 0);
    }

    println!("Sum: {}", sum_of_all_powers);
}

fn part_one() {
    let binding = read_file("day2input.txt");
    let input_lines = binding.split("\r\n").into_iter();
    
    let mut global_game_is_already_invalid = false;
    
    let mut sum_of_valid_games = 0;

    for input in input_lines {
        let Some((game_number, game_set)) = input.split_once(":") else { continue };
        let Some((_,game_number)) = game_number.split_once("Game ") else { continue; };
        
        let Ok(game_number) = game_number.parse::<usize>() else { continue; }; 
        let game_set_results = game_set.split(";");

        for set_result in game_set_results {
            if global_game_is_already_invalid {
                break;
            }        
    
            let set_results = set_result.trim();
            let results = set_results.split(",");
            
            for result in results {
                let result = result.trim();

                let mut num_color_split = result.split(" ");
                let amount = num_color_split.next().unwrap();
                let color = num_color_split.next().unwrap();

                let amount = amount.parse::<usize>().unwrap();
                match color {
                    "red" => {
                        if amount > POSSIBLE_VALUES_RGB[0].into() {
                            global_game_is_already_invalid = true;
                            break;
                        }
                    },
                    "green" => {
                        if amount > POSSIBLE_VALUES_RGB[1].into() {
                            global_game_is_already_invalid = true;
                            break;
                        }
                    },
                    "blue" => {
                        if amount > POSSIBLE_VALUES_RGB[2].into() {
                            global_game_is_already_invalid = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        // reset_tmp_arr(&mut tmp_sum_of_revealed_values);

        if global_game_is_already_invalid {
            global_game_is_already_invalid = false;
            continue;
        } else {
            sum_of_valid_games += game_number;
        }
    }

    println!("{}", sum_of_valid_games);

}

fn reset_tmp_arr(tmp_arr: &mut Vec<usize>) {
    tmp_arr[0] = 0;
    tmp_arr[1] = 0;
    tmp_arr[2] = 0;
}

