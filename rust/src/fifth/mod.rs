use regex::Regex;

use self::cargo_stack::CargoStack;

mod cargo_stack;

pub fn main(input_str: &String) {
    let mut input_list = input_str.split("\n");

    let container_regex = Regex::new(r"([A-Z]+)").unwrap();
    let move_regex = Regex::new(r"([0-9]+)").unwrap();

    let mut curr_input = input_list.next().unwrap();

    let num_stacks = (curr_input.len() + 1) / 4;

    let mut vecs = Vec::new();

    for _ in 0..num_stacks {
        vecs.push(Vec::new());
    }

    let mut cargo_stacks = Vec::new();

    while curr_input.contains("[") {
        let mut remaining = curr_input;

        'inner: for i in 0..num_stacks {
            let current;
            if remaining.len() < 4 {
                current = remaining;
            } else {
                let remain;
                (current, remain) = remaining.split_at(4);
                remaining = remain;
            }

            if current.trim().is_empty() {
                continue 'inner;
            }

            vecs.get_mut(i)
                .expect("Found more stacks than expected")
                .push(
                    container_regex
                        .captures(current)
                        .expect(&format!("expected to capture a char: \"{}\"", current))
                        .get(1)
                        .expect("expected to retrieve a first capture")
                        .as_str()
                        .chars()
                        .next()
                        .expect("expected the match to not be empty"),
                );
        }

        curr_input = input_list.next().unwrap();
    }

    for vec in &vecs {
        cargo_stacks.push(CargoStack::new(vec.clone()));
    }

    let mut cargo_stacks_in_order = cargo_stacks.clone();

    for input in input_list {
        if input.starts_with("move") {
            let numbers: Vec<regex::Captures> = move_regex.captures_iter(input).collect();

            let amount: u16 = numbers
                .get(0)
                .expect("Expected to have one number match")
                .get(1)
                .expect("Expected to have one match")
                .as_str()
                .parse()
                .expect("Expected to find a number");

            let from: usize = numbers
                .get(1)
                .expect("Expected to have two number matches")
                .get(1)
                .expect("Expected to have one match")
                .as_str()
                .parse()
                .expect("Expected to find a number");

            let to: usize = numbers
                .get(2)
                .expect("Expected to have three number matches")
                .get(1)
                .expect("Expected to have one match")
                .as_str()
                .parse()
                .expect("Expected to find a number");

            let moved_containers = cargo_stacks
                .get_mut(from - 1)
                .expect("Expected more cargo stacks, than exist")
                .remove(amount);

            cargo_stacks
                .get_mut(to - 1)
                .expect("Expected more cargo stacks, than exist")
                .add(moved_containers.clone());

            let moved_containers = cargo_stacks_in_order
                .get_mut(from - 1)
                .expect("Expected more cargo stacks, than exist")
                .remove(amount);

            cargo_stacks_in_order
                .get_mut(to - 1)
                .expect("Expected more cargo stacks, than exist")
                .add_in_order(moved_containers.clone());
        }
    }

    println!(
        "Top containers: {}",
        cargo_stacks
            .iter()
            .flat_map(|x| vec![x.get_top()])
            .collect::<String>()
    );

    println!(
        "Top containers with new crane: {}",
        cargo_stacks_in_order
            .iter()
            .flat_map(|x| vec![x.get_top()])
            .collect::<String>()
    );
}
