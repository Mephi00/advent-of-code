pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i128 {
    let mut content = input_str.clone();

    content.retain(|c| c.is_digit(10) || c == '\n');

    let mut total = 0;

    for line in content.split("\n") {
        if line.is_empty() {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();

        total += format!("{}{}", chars.first().unwrap(), chars.last().unwrap())
            .parse::<i128>()
            .unwrap();
    }

    total
}

fn part_two(input_str: &String) -> i128 {
    let mut total = 0;

    for line in input_str.split("\n") {
        if line.is_empty() {
            continue;
        }

        let line_string = parse_line_words(line);
        let chars: Vec<char> = line_string.chars().collect();

        let assembled = format!("{}{}", chars.first().unwrap(), chars.last().unwrap());

        total += assembled.parse::<i128>().unwrap();
    }

    total
}

fn parse_line_words(lin: &str) -> String {
    struct SortStruct<'a> {
        pub idx: usize,
        pub repl_str: &'a str,
    }

    impl<'a> SortStruct<'a> {
        pub fn create(lin: &str, num: &str, repl_str: &'a str) -> Vec<Self> {
            let mut ret = Vec::new();
            for (idx, _) in lin.match_indices(num) {
                ret.push(Self { idx, repl_str });
            }
            ret
        }
    }

    let mut indices = Vec::new();
    indices.push(SortStruct::create(lin, "one", "1"));
    indices.push(SortStruct::create(lin, "two", "2"));
    indices.push(SortStruct::create(lin, "three", "3"));
    indices.push(SortStruct::create(lin, "four", "4"));
    indices.push(SortStruct::create(lin, "five", "5"));
    indices.push(SortStruct::create(lin, "six", "6"));
    indices.push(SortStruct::create(lin, "seven", "7"));
    indices.push(SortStruct::create(lin, "eight", "8"));
    indices.push(SortStruct::create(lin, "nine", "9"));

    indices.retain(|s| !s.is_empty());

    let mut matches = indices
        .iter()
        .flat_map(|op| op)
        .collect::<Vec<&SortStruct<'_>>>();

    matches.sort_by_key(|f| f.idx);

    let mut ret = "".to_string();
    for (idx, c) in lin.chars().enumerate() {
        if c.is_digit(10) {
            ret = format!("{}{}", ret, c);
        } else {
            if let Ok(index) = matches.binary_search_by_key(&idx, |f| f.idx) {
                ret = format!("{}{}", ret, matches.get(index).unwrap().repl_str);
            }
        }
    }

    ret.to_string()
}
