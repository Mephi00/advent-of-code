use std::fs::read_to_string;

use crate::elve::Elve;

mod elve;

fn main() -> Result<(), std::io::Error> {
    let input_str = read_to_string("input.txt")?;

    let input_list: Vec<&str> = input_str.split("\n").collect();

    let mut elves = Vec::new();

    let mut buff = Vec::new();
    for item in input_list {
        if !item.is_empty() {
            buff.push(item.parse::<u32>().expect("couldnt parse input as u32"));
        } else {
            elves.push(Elve::new(buff.clone()));
            buff = Vec::new();
        }
    }

    let max = elves.iter().max().unwrap().total;
    let top_three_total = calc_top_three_total(elves);

    println!(
        "single max value: {}\ntop three total: {}",
        max, top_three_total
    );

    Ok(())
}

fn calc_top_three_total(mut elves: Vec<Elve>) -> u32 {
    elves.sort_by(|a, b| b.cmp(a));

    let mut total = 0;
    for i in 0..3 {
        total += elves[i].total;
    }

    total
}
