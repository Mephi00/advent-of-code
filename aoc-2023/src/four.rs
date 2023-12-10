pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i32 {
    input_str
        .lines()
        .map(|line| {
            let (expect, actual) = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>()[1]
                .split_once("|")
                .map(|(f1, f2)| (f1.split_whitespace(), f2.split_whitespace()))
                .map(|(f1, f2)| {
                    (
                        f1.map(|s| s.trim().parse().unwrap()),
                        f2.map(|s| s.trim().parse().unwrap()),
                    )
                })
                .unwrap();

            let expect_list: Vec<i32> = expect.collect();

            let num_matches = actual.filter(|s| expect_list.contains(s)).count();

            if num_matches <= 2 {
                return num_matches.try_into().unwrap();
            }

            2_i32.pow(TryInto::<u32>::try_into(num_matches).unwrap() - 1)
        })
        .sum()
}

fn part_two(input_str: &String) -> usize {
    let list_of_wins: Vec<usize> = input_str
        .lines()
        .map(|line| {
            let (expect, actual) = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>()[1]
                .split_once("|")
                .map(|(f1, f2)| (f1.split_whitespace(), f2.split_whitespace()))
                .map(|(f1, f2)| {
                    (
                        f1.map(|s| s.trim().parse().unwrap()),
                        f2.map(|s| s.trim().parse().unwrap()),
                    )
                })
                .unwrap();

            let expect_list: Vec<i32> = expect.collect();

            actual.filter(|s| expect_list.contains(s)).count()
        })
        .collect();

    let mut total_cards = 0;

    for idx in 0..list_of_wins.len() {
        let total = find_num_cards(&list_of_wins, idx);

        total_cards += total;
    }

    total_cards
}

fn find_num_cards(list: &Vec<usize>, card: usize) -> usize {
    if list.len() - 1 <= card || list[card] == 0 {
        return 1;
    }

    let mut total = 1;

    for idx in 1..list[card] + 1 {
        total += find_num_cards(list, card + idx);
    }

    total
}
