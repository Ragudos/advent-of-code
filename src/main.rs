use std::fs;

fn main() {
    let binding = read_file("day1input.txt");
    let input_contents = binding.split("\r\n").into_iter();    
    let mut sum: usize = 0;

    for string in input_contents {
        let mut first_digit = String::with_capacity(1);
        let mut last_digit = String::with_capacity(1);

        println!("string: {}", string);

        if let Some(converted_string) = find_first_named_int(string) {
            let char_val = converted_string.chars().next().unwrap();

            first_digit.push(char_val);
        } else {
            for (idx, char) in string.chars().enumerate() {
                if char.is_numeric() {
                    first_digit.push(char);
                    break;
                }

                let sliced_string = String::from(string).split_off(idx);
                let Some(first_named_int) = find_first_named_int(&sliced_string) else { continue };

                first_digit.push_str(&first_named_int);
                break;
            }
        }

        if let Some(converted_string) = find_last_named_int(string) {
            let char_val = converted_string.chars().next().unwrap();

            last_digit.push(char_val);
        } else {
            for (idx, char) in string.chars().rev().enumerate() {
                if char.is_numeric() {
                    last_digit.push(char);
                    break;
                }

                let mut reversed_string = string.chars().rev().collect::<String>();
                let reversed_split_string = reversed_string.split_off(idx);
                let sliced_string = reversed_split_string.chars().rev().collect::<String>();
                let Some(last_named_int) = find_last_named_int(&sliced_string) else { continue };

                last_digit.push_str(&last_named_int);
                break;
            }
        }

        let mut combined_digit = String::with_capacity(2);
        
        combined_digit.push_str(&first_digit);
        combined_digit.push_str(&last_digit);

        println!("First digit: {:?}, Last digit: {:?}, Combined digit: {:?}", first_digit, last_digit, combined_digit);

        let Ok(parsed_digit) = combined_digit.parse::<usize>() else { continue; };

        sum += parsed_digit;
    }

    println!("\nSum: {:?}\n", sum);
}

fn find_last_named_int(string_int: &str) -> Option<String> {
    if string_int.ends_with("one") {
        Some(String::from("1"))
    } else if string_int.ends_with("two") {
        Some(String::from("2"))
    } else if string_int.ends_with("three") {
        Some(String::from("3"))
    } else if string_int.ends_with("four"){
        Some(String::from("4"))
    } else if string_int.ends_with("five") {
        Some(String::from("5"))
    } else if string_int.ends_with("six") {
        Some(String::from("6"))
    } else if string_int.ends_with("seven") {
        Some(String::from("7"))
    } else if string_int.ends_with("eight") {
        Some(String::from("8"))
    } else if string_int.ends_with("nine") {
        Some(String::from("9"))
    } else {
        None
    }
}

fn find_first_named_int(string_int: &str) -> Option<String> {
    if string_int.starts_with("one") {
        Some(String::from("1"))
    } else if string_int.starts_with("two") {
        Some(String::from("2"))
    } else if string_int.starts_with("three") {
        Some(String::from("3"))
    } else if string_int.starts_with("four") {
        Some(String::from("4"))
    } else if string_int.starts_with("five") {
        Some(String::from("5"))
    } else if string_int.starts_with("six") {
        Some(String::from("6"))
    } else if string_int.starts_with("seven") {
        Some(String::from("7"))
    } else if string_int.starts_with("eight") {
       Some(String::from("8"))
    } else if string_int.starts_with("nine") {
        Some(String::from("9"))
    } else {
        None
    }
}

fn read_file(path_to_file: &'static str) -> String {
    let input_contents = fs::read_to_string(path_to_file)
        .expect("Should have been able to read the file.");

    input_contents
}

