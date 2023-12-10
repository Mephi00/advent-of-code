pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i64 {
    let mut total = 1;
    let lines: Vec<&str> = input_str.lines().collect();

    let times = get_numbers(lines[0]);
    let distances = get_numbers(lines[1]);

    for i in 0..times.len() {
        total *= calc_options(times[i], distances[i]);
    }

    total
}

fn part_two(input_str: &String) -> i64 {
    let lines: Vec<&str> = input_str.lines().collect();

    let times = get_numbers(lines[0]);
    let distances = get_numbers(lines[1]);
    let time: i64 = times
        .iter()
        .fold(0, |curr, new| format!("{curr}{new}").parse().unwrap());

    let distance: i64 = distances
        .iter()
        .fold(0, |curr, new| format!("{curr}{new}").parse().unwrap());

    calc_options(time, distance)
}

fn calc_options(time: i64, distance: i64) -> i64 {
    let mut lower_bound = 0;
    for hold_time in 1..time {
        if hold_time * (time - hold_time) - distance > 0 {
            lower_bound = hold_time;
            break;
        }
    }

    let mut upper_bound = 0;
    for hold_time in (1..time).rev() {
        if hold_time * (time - hold_time) - distance > 0 {
            upper_bound = hold_time;
            break;
        }
    }

    1 + upper_bound - lower_bound
}

fn get_numbers(line: &str) -> Vec<i64> {
    let mut ret = Vec::new();
    for split in line.split_whitespace().map(|s| s.trim()) {
        if !split.ends_with(":") && !split.is_empty() {
            ret.push(split.parse().unwrap());
        }
    }

    ret
}

// struct Race {
//     pub duration: i64,
//     pub distance_to_win: i64,
// }

// impl Race {
//     pub fn find_margin_of_error(&self) -> i64 {}
// }
