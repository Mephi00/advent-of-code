use crate::fourth::pair::Pair;

mod elf;
mod pair;

pub fn main(input_str: &String) {
    let pairs_str = input_str.split("\n");

    let mut total_overlapping = 0;
    let mut partly_overlapping = 0;

    for pair_str in pairs_str {
        if pair_str.is_empty() {
            continue;
        }
        let pair = Pair::new(pair_str);

        if pair.complete_overlapping() {
            total_overlapping += 1;
        }

        if pair.partly_overlapping() {
            partly_overlapping += 1;
        }
    }

    println!(
        "Pars with totally overlapping sections: {}",
        total_overlapping
    );

    println!(
        "Pars with partly overlapping sections: {}",
        partly_overlapping
    );
}
