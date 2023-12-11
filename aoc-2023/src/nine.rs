pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i64 {
    input_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|n| extrapolate_next(&n))
        .sum()
}

fn part_two(input_str: &String) -> i64 {
    input_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|n| extrapolate_first(&n))
        .sum()
}

fn extrapolate_next(numbers: &[i64]) -> i64 {
    let mut all_vec: Vec<Vec<i64>> = Vec::new();
    all_vec.push(numbers.into());

    let mut curr_vec = Vec::new();
    while !all_vec.last().unwrap().iter().all(|n| *n == 0) {
        let working_vec = all_vec.last().unwrap();
        for i in 0..working_vec.len() {
            if i + 1 == working_vec.len() {
                continue;
            }
            curr_vec.push(working_vec[i + 1] - working_vec[i]);
        }

        all_vec.push(curr_vec);
        curr_vec = vec![];
    }

    let mut curr_append = 0;
    for (idx, it) in all_vec.iter().rev().enumerate() {
        if idx == 0 {
            continue;
        }

        curr_append += it.last().unwrap();
    }

    curr_append
}

fn extrapolate_first(numbers: &[i64]) -> i64 {
    let mut all_vec: Vec<Vec<i64>> = Vec::new();
    all_vec.push(numbers.into());

    let mut curr_vec = Vec::new();
    while !all_vec.last().unwrap().iter().all(|n| *n == 0) {
        let working_vec = all_vec.last().unwrap();
        for i in 0..working_vec.len() {
            if i + 1 == working_vec.len() {
                continue;
            }
            curr_vec.push(working_vec[i + 1] - working_vec[i]);
        }

        all_vec.push(curr_vec);
        curr_vec = vec![];
    }

    let mut curr_prepend = 0;
    for (idx, it) in all_vec.iter().rev().enumerate() {
        if idx == 0 {
            continue;
        }

        curr_prepend = it.first().unwrap() - curr_prepend;
    }

    curr_prepend
}
