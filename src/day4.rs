use std::collections::HashMap;

use crate::utils::read_file;


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

}

fn part1() {
    let input = read_file("day4test.txt");
    let contents = input.split("\r\n").filter(|x| !x.is_empty()).map(|x| {
        let cards = x.split(':').collect::<Vec<_>>();
        cards[1].split('|').collect::<Vec<_>>()
    });

    let mut points = 0;

    for (idx, card) in contents.enumerate() {
        let winning_card_string = card.get(0).unwrap().trim();
        let winning_cards = winning_card_string.split(' ');
        let mut hashmap_of_winning_cards: HashMap<usize, bool> = HashMap::new();

        for winning_card in winning_cards {
            let winning_card = winning_card.trim();
            let winning_card = winning_card.parse::<usize>().unwrap();
            hashmap_of_winning_cards.insert(winning_card, true);
        }

        let held_cards = card.get(1).unwrap().trim();
        let held_cards = held_cards.split(' ');
        let mut points_for_this_card = 0;

        for held_card in held_cards {
            let held_card = held_card.trim();

            if held_card.is_empty() {
                continue;
            }

            let held_card = held_card.parse::<usize>().unwrap();

            if hashmap_of_winning_cards.contains_key(&held_card) {
                if points_for_this_card == 0 {
                    points_for_this_card = 1;
                } else if idx == 0 {
                   points_for_this_card <<= 1;
                } else {
                    points_for_this_card += 1;
                }
            }
        }

        points += points_for_this_card;
    }

    println!("Points: {}", points);
}

