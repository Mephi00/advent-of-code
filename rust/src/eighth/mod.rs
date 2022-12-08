pub fn main(input_str: &String) {
    let mut grid = Vec::new();

    let input_rows: Vec<&str> = input_str.split("\n").map(|x| x.trim()).collect();

    for row in input_rows {
        let trees: Vec<i8> = row
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect(&format!("Couldnt parse {}", x)))
            .collect();

        if !trees.is_empty() {
            grid.push(trees);
        }
    }

    println!("grid size: {}x{}", grid.len(), grid[0].len());

    let mut visible_trees = 0;
    let mut top_scenic_score = 0;
    let mut factorss = (0, 0, 0, 0);
    let mut location: (usize, usize) = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, tree) in row.iter().enumerate() {
            let mut comp_right = false;
            let mut visible_left = true;
            let mut visible_right = true;
            let mut scenic_buf = 0;
            let mut viewing_distance = 0;

            let mut factors = (0, 0, 0, 0);

            for (idx, comp_tree) in row.iter().enumerate() {
                if idx == column_index {
                    if visible_left {
                        viewing_distance = column_index;
                    }
                    scenic_buf = viewing_distance;
                    factors.2 = viewing_distance;
                    viewing_distance = 0;
                    comp_right = true;
                    continue;
                }

                if !visible_right && comp_right {
                    break;
                }

                if tree <= comp_tree {
                    if comp_right {
                        viewing_distance += 1;
                        visible_right = false;
                    } else {
                        viewing_distance = column_index - idx;
                        visible_left = false;
                    }
                } else if comp_right {
                    viewing_distance += 1;
                }
            }

            scenic_buf *= viewing_distance;
            factors.3 = viewing_distance;

            viewing_distance = 0;

            let mut comp_bottom = false;
            let mut visible_top = true;
            let mut visible_bottom = true;
            for i in 0..grid.len() {
                if i == row_index {
                    if visible_top {
                        viewing_distance = row_index;
                    }

                    scenic_buf *= viewing_distance;
                    factors.0 = viewing_distance;
                    viewing_distance = 0;
                    comp_bottom = true;
                    continue;
                }

                if !visible_bottom && comp_bottom {
                    break;
                }

                if tree <= &grid[i][column_index] {
                    if comp_bottom {
                        viewing_distance += 1;
                        visible_bottom = false;
                    } else {
                        viewing_distance = row_index - i;
                        visible_top = false;
                    }
                } else if comp_bottom {
                    viewing_distance += 1;
                }
            }

            scenic_buf *= viewing_distance;
            factors.1 = viewing_distance;

            if visible_bottom || visible_top || visible_left || visible_right {
                visible_trees += 1;
            }

            if scenic_buf > top_scenic_score {
                top_scenic_score = scenic_buf;
                factorss = factors;
                location = (row_index, column_index);
            }
        }
    }

    println!("number of visible trees: {}", visible_trees);
    println!("top scenic score: {}", top_scenic_score);
    println!("factors: {:?}", factorss);
    println!("location: {:?}", location);
}
