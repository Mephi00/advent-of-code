use crate::third::{group::Group, rucksack::Rucksack};

mod group;
mod rucksack;

pub fn main(input_str: &String) {
    let rucksacks_str = input_str.split("\n");

    let mut prio_sum = 0;

    let mut group_prio_sum = 0;

    let mut current_group = Vec::new();

    for rucksack_str in rucksacks_str {
        if !rucksack_str.is_empty() {
            let rucksack = Rucksack::new(rucksack_str);

            prio_sum += rucksack.priority_total;

            current_group.push(rucksack);

            if current_group.len() == 3 {
                group_prio_sum += Group::new((
                    current_group[0].clone(),
                    current_group[1].clone(),
                    current_group[2].clone(),
                ))
                .identifier_prio;

                current_group = Vec::new();
            }
        }
    }

    println!("total priorities: {:#?}", prio_sum);
    println!(
        "total pririority of the group identifier: {:#?}",
        group_prio_sum
    );
}
