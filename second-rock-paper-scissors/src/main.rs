use std::fs::read_to_string;

use move_mod::{Match, Move};

use crate::move_mod::{EndResult, MatchByResult};

mod move_mod;

fn main() {
    let input_str = read_to_string("input.txt").expect("expected to read input.txt");

    let input_list: Vec<&str> = input_str.split('\n').collect();

    println!("amount of lines: {}", input_list.len());

    let mut matches = Vec::new();
    let mut matches_by_result = Vec::new();

    for pair in input_list {
        if !pair.is_empty() {
            let chars: Vec<char> = pair.chars().collect();

            matches.push(Match(
                Move::new(chars[0]).expect(&format!("couldnt convert {}", chars[0])),
                Move::new(chars[2]).expect(&format!("couldnt convert {}", chars[2])),
            ));

            matches_by_result.push(MatchByResult(
                Move::new(chars[0]).expect(&format!("couldnt convert {}", chars[0])),
                EndResult::new(chars[2]).expect(&format!("couldnt convert {} to result", chars[0])),
            ))
        }
    }
    println!(
        "total: {}",
        matches.iter().map(|a| a.get_result()).sum::<u16>()
    );

    println!(
        "total by result: {}",
        matches_by_result
            .iter()
            .map(|a| a.get_result())
            .sum::<u16>()
    );
}
