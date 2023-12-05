use crate::utils::read_file;

static POSSIBLE_VALUES_RGB: [u8; 3] = [12, 13, 14];

pub fn main() {
    let binding = read_file("day2input.txt");
    let input_lines = binding.split("\r\n").into_iter();
    
    let mut global_game_is_already_invalid = false;
    
    let mut tmp_sum_of_revealed_values: Vec<usize> = Vec::with_capacity(3);
        tmp_sum_of_revealed_values.push(0);
        tmp_sum_of_revealed_values.push(0);
        tmp_sum_of_revealed_values.push(0);

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

