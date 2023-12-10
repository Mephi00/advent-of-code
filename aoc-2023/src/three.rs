use std::str::Lines;

pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i64 {
    let mut total = 0;

    let grid = build_grid(input_str.lines());

    for (line_idx, line) in input_str.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                for line in grid.get(line_idx).unwrap() {
                    if line.is_hit(true, c_idx) {
                        total += line.value;
                    }
                }
                if line_idx != 0 {
                    for line in grid.get(line_idx - 1).unwrap() {
                        if line.is_hit(false, c_idx) {
                            total += line.value;
                        }
                    }
                }

                if line_idx < grid.len() - 1 {
                    for line in grid.get(line_idx + 1).unwrap() {
                        if line.is_hit(false, c_idx) {
                            total += line.value;
                        }
                    }
                }
            }
        }
    }

    total
}

fn part_two(input_str: &String) -> i64 {
    let mut total = 0;

    let grid = build_grid(input_str.lines());

    for (line_idx, line) in input_str.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c == '*' {
                let mut hits = vec![];
                for line in grid.get(line_idx).unwrap() {
                    if line.is_hit(true, c_idx) {
                        hits.push(line.value);
                    }
                }
                if line_idx != 0 {
                    for line in grid.get(line_idx - 1).unwrap() {
                        if line.is_hit(false, c_idx) {
                            hits.push(line.value);
                        }
                    }
                }

                if line_idx < grid.len() - 1 {
                    for line in grid.get(line_idx + 1).unwrap() {
                        if line.is_hit(false, c_idx) {
                            hits.push(line.value);
                        }
                    }
                }

                if hits.len() == 2 {
                    total += hits[0] * hits[1];
                }
            }
        }
    }

    total
}

fn build_grid(lines: Lines<'_>) -> Vec<Vec<LineNum>> {
    let mut grid: Vec<Vec<LineNum>> = Vec::new();

    for line in lines {
        let mut line_nums = vec![];
        let mut temp_num = "".to_string();
        for (idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                temp_num.push(c);
            } else if !temp_num.is_empty() {
                let numeric_value = temp_num.parse().unwrap();
                let line_num = LineNum {
                    start: idx - temp_num.len(),
                    end: idx - 1,
                    value: numeric_value,
                };

                line_nums.push(line_num);

                temp_num = "".to_string();
            }
        }

        let len = line.chars().collect::<Vec<char>>().len();

        if !temp_num.is_empty() {
            let numeric_value = temp_num.parse().unwrap();
            let line_num = LineNum {
                start: len - temp_num.len(),
                end: len - 1,
                value: numeric_value,
            };

            line_nums.push(line_num);
        }

        grid.push(line_nums);
    }

    grid
}

#[derive(Debug)]
struct LineNum {
    pub start: usize,
    pub end: usize,
    pub value: i64,
}

impl LineNum {
    pub fn is_hit(&self, same_line: bool, idx: usize) -> bool {
        let start_limit;

        if self.start == 0 {
            start_limit = 0;
        } else {
            start_limit = self.start - 1;
        }

        if same_line {
            return idx == start_limit || idx == self.end + 1;
        }

        idx >= start_limit && idx <= self.end + 1
    }
}
