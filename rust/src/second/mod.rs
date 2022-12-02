use models::{end_result::EndResult, match_by_result::MatchByResult, r#match::Match, r#move::Move};

mod models;

pub fn main(input_str: &String) {
    let input_list: Vec<&str> = input_str.split('\n').collect();

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
        "total score: {}",
        matches.iter().map(|a| a.get_result()).sum::<u16>()
    );

    println!(
        "total score by expected result: {}",
        matches_by_result
            .iter()
            .map(|a| a.get_result())
            .sum::<u16>()
    );
}
