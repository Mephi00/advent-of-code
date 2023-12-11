pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> usize {
    let grid: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();

    create_loop(&grid).len() / 2
}

fn part_two(input_str: &String) -> usize {
    0
}

fn create_loop(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut loop_to_run = vec![];
    let start_y = grid
        .iter()
        .enumerate()
        .find_map(|(idx, v)| {
            if v.contains(&'S') {
                return Some(idx);
            }
            None
        })
        .unwrap();

    let start_x = grid[start_y]
        .iter()
        .enumerate()
        .find_map(|(idx, ch)| {
            if *ch == 'S' {
                return Some(idx);
            }
            None
        })
        .unwrap();
    let mut curr_y = start_y;
    let mut curr_x = start_x;
    let mut prev_y = start_y;
    let mut prev_x = start_x;
    let mut curr_char = 'S';

    let mut start = true;
    while start || !(curr_x == start_x && curr_y == start_y) {
        let temp_x = curr_x;
        let temp_y = curr_y;
        start = false;
        if curr_char == 'S' {
            if curr_y != grid.len() - 1 {
                match grid[curr_y + 1][curr_x] {
                    'J' => {
                        curr_y += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    '|' => {
                        curr_y += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    'L' => {
                        curr_y += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    _ => {}
                }
            }

            if curr_y != 0 {
                match grid[curr_y - 1][curr_x] {
                    'F' => {
                        curr_y -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    '|' => {
                        curr_y -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    '7' => {
                        curr_y -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    _ => {}
                }
            }

            if curr_x != 0 {
                match grid[curr_y][curr_x - 1] {
                    'F' => {
                        curr_x -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    '-' => {
                        curr_x -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    'L' => {
                        curr_x -= 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    _ => {}
                }
            }

            if curr_x != grid[curr_y].len() - 1 {
                match grid[curr_y][curr_x + 1] {
                    '7' => {
                        curr_x += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    '-' => {
                        curr_x += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    'J' => {
                        curr_x += 1;
                        curr_char = grid[curr_y][curr_x];
                        loop_to_run.push((curr_y, curr_x));
                        continue;
                    }
                    _ => {}
                }
            }
        } else {
            if prev_y < curr_y {
                match curr_char {
                    '|' => {
                        curr_y += 1;
                    }
                    'L' => {
                        curr_x += 1;
                    }
                    'J' => {
                        curr_x -= 1;
                    }
                    _ => {}
                }
            } else if prev_y > curr_y {
                match curr_char {
                    '|' => {
                        curr_y -= 1;
                    }
                    '7' => {
                        curr_x -= 1;
                    }
                    'F' => {
                        curr_x += 1;
                    }
                    _ => {}
                }
            } else {
                if prev_x < curr_x {
                    match curr_char {
                        '-' => {
                            curr_x += 1;
                        }
                        '7' => {
                            curr_y += 1;
                        }
                        'J' => {
                            curr_y -= 1;
                        }
                        _ => {}
                    }
                } else if prev_x > curr_x {
                    match curr_char {
                        '-' => {
                            curr_x -= 1;
                        }
                        'F' => {
                            curr_y += 1;
                        }
                        'L' => {
                            curr_y -= 1;
                        }
                        _ => {}
                    }
                } else {
                    panic!(
                        "stopped with {curr_char} at {curr_y}, {curr_x} prev {}",
                        grid[prev_y][prev_x]
                    );
                }
            }
        }
        prev_x = temp_x;
        prev_y = temp_y;
        curr_char = grid[curr_y][curr_x];
        loop_to_run.push((curr_y, curr_x));
    }

    loop_to_run
}
