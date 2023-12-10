use std::collections::HashMap;

pub fn exec(input_str: String) {
    println!("{}", greatest_common_factor(&[143, 65, 13 * 234782]));
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> usize {
    let mut total = 0;
    let instructions = input_str.split_once("\n").unwrap().0;
    let nodes: HashMap<String, (String, String)> = input_str
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.contains("="))
        .map(|line| {
            let elements: Vec<String> = line
                .split("=")
                .flat_map(|s| {
                    s.trim()
                        .split(",")
                        .map(|ss| ss.trim().replace("(", "").replace(")", ""))
                })
                .collect();

            (
                elements[0].clone(),
                (elements[1].clone(), elements[2].clone()),
            )
        })
        .collect();

    let mut found = false;

    let mut curr = "AAA".to_string();
    while !found {
        for instruction in instructions.chars() {
            if curr == "ZZZ".to_string() {
                found = true;
                break;
            }
            total += 1;

            curr = match instruction {
                'L' => nodes.get(&curr).unwrap().0.clone(),
                'R' => nodes.get(&curr).unwrap().1.clone(),
                _ => curr,
            }
        }
    }

    total
}

fn part_two(input_str: &String) -> usize {
    let instructions = input_str.split_once("\n").unwrap().0;
    let nodes: HashMap<String, (String, String)> = input_str
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.contains("="))
        .map(|line| {
            let elements: Vec<String> = line
                .split("=")
                .flat_map(|s| {
                    s.trim()
                        .split(",")
                        .map(|ss| ss.trim().replace("(", "").replace(")", ""))
                })
                .collect();

            (
                elements[0].clone(),
                (elements[1].clone(), elements[2].clone()),
            )
        })
        .collect();

    let mut curr_vec = Vec::new();
    let mut results = Vec::new();

    for (key, _) in nodes.iter() {
        if key.ends_with("A") {
            curr_vec.push(key.clone());
        }
    }

    for i in 0..curr_vec.len() {
        let mut total = 0;
        'outer: loop {
            for instruction in instructions.chars() {
                total += 1;
                match instruction {
                    'L' => curr_vec[i] = nodes.get(&curr_vec[i]).unwrap().0.clone(),
                    'R' => curr_vec[i] = nodes.get(&curr_vec[i]).unwrap().1.clone(),
                    _ => {}
                };

                if curr_vec[i].ends_with("Z") {
                    results.push(total);
                    break 'outer;
                }
            }
        }
    }

    largest_common_multiplier(&results)
}

fn largest_common_multiplier(numbers: &[usize]) -> usize {
    let gg_t = greatest_common_factor(numbers);
    gg_t * numbers.iter().map(|n| n / gg_t).product::<usize>()
}

fn greatest_common_factor(numbers: &[usize]) -> usize {
    let mut gg_t = numbers[0];
    for (a, b) in numbers.iter().enumerate() {
        if a == 0 {
            continue;
        }

        let mut b = *b;
        while b % gg_t != 0 {
            let a = gg_t;
            gg_t = b % gg_t;
            b = a;
        }
    }

    gg_t
}
