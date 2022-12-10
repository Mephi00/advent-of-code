use self::uniqu_vec::UniqueVec;

mod uniqu_vec;

pub fn main(input_str: &String) {
    let moves: Vec<&str> = input_str.split("\n").map(|x| x.trim()).collect();

    println!(
        "Number of unique places of the tail with len 1: {}",
        simulate_moves(1, moves.clone())
    );

    println!(
        "Number of unique places of the tail with len 9: {}",
        simulate_moves(9, moves)
    );
}

fn simulate_moves(num_pieces: i32, moves: Vec<&str>) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_positions = UniqueVec::new();

    let mut pieces_pos = Vec::new();

    for _ in 0..num_pieces {
        pieces_pos.push((0, 0));
    }

    tail_positions.push(pieces_pos[pieces_pos.len() - 1]);

    let start_pos = head_pos;

    for mov in moves {
        if mov.is_empty() {
            continue;
        }

        let mov_parts: Vec<&str> = mov.split_ascii_whitespace().collect();

        let amount: i32 = mov_parts[1]
            .parse()
            .expect(&format!("couldnt parse {}", mov_parts[1]));

        match mov_parts[0] {
            "U" => handle_movements(
                &mut pieces_pos,
                &mut tail_positions,
                amount,
                &start_pos,
                || {
                    head_pos.0 += 1;
                    head_pos
                },
            ),
            "D" => handle_movements(
                &mut pieces_pos,
                &mut tail_positions,
                amount,
                &start_pos,
                || {
                    head_pos.0 -= 1;
                    head_pos
                },
            ),
            "R" => handle_movements(
                &mut pieces_pos,
                &mut tail_positions,
                amount,
                &start_pos,
                || {
                    head_pos.1 += 1;
                    head_pos
                },
            ),
            "L" => handle_movements(
                &mut pieces_pos,
                &mut tail_positions,
                amount,
                &start_pos,
                || {
                    head_pos.1 -= 1;
                    head_pos
                },
            ),
            _ => {
                panic!("Couldnt determin moving direction of {}", mov)
            }
        }
    }

    tail_positions.len()
}

fn tail_movement(
    head_pos: &(i32, i32),
    tail_pos: &(i32, i32),
    unique_counter: Option<&mut UniqueVec<(i32, i32)>>,
) -> (i32, i32) {
    let mut curr_tail_pos = tail_pos.clone();

    // head above the tail
    if head_pos.0 > tail_pos.0 + 1 {
        curr_tail_pos.0 += 1;
        // head is diagonale left
        if head_pos.1 < tail_pos.1 {
            curr_tail_pos.1 -= 1;
        }

        // head is diagonale right
        if head_pos.1 > tail_pos.1 {
            curr_tail_pos.1 += 1;
        }
    } else
    // head below the tail
    if head_pos.0 < tail_pos.0 - 1 {
        curr_tail_pos.0 -= 1;
        // head is diagonale left
        if head_pos.1 < tail_pos.1 {
            curr_tail_pos.1 -= 1;
        }

        // head is diagonale right
        if head_pos.1 > tail_pos.1 {
            curr_tail_pos.1 += 1;
        }
    } else
    // head left of tail
    if head_pos.1 < tail_pos.1 - 1 {
        curr_tail_pos.1 -= 1;
        // head is diagonale top
        if head_pos.0 > tail_pos.0 {
            curr_tail_pos.0 += 1;
        }

        //head is diagonale bottom
        if head_pos.0 < tail_pos.0 {
            curr_tail_pos.0 -= 1;
        }
    } else
    //head right of tail
    if head_pos.1 > tail_pos.1 + 1 {
        curr_tail_pos.1 += 1;
        // head is diagonale top
        if head_pos.0 > tail_pos.0 {
            curr_tail_pos.0 += 1;
        }

        //head is diagonale bottom
        if head_pos.0 < tail_pos.0 {
            curr_tail_pos.0 -= 1;
        }
    }

    if unique_counter.is_some() {
        unique_counter.unwrap().push(curr_tail_pos);
    }

    curr_tail_pos
}

fn handle_movements<FN>(
    body_pos: &mut Vec<(i32, i32)>,
    tail_positions: &mut UniqueVec<(i32, i32)>,
    amount: i32,
    // purely for a nice debugging experience
    #[allow(dead_code, unused_variables)] start_pos: &(i32, i32),
    mut mod_head: FN,
) where
    FN: FnMut() -> (i32, i32),
{
    // let mut head = (0, 0);
    for _ in 1..amount + 1 {
        let curr_head = mod_head();
        for i in 0..body_pos.len() {
            if i == body_pos.len() - 1 {
                body_pos[i] = tail_movement(&body_pos[i - 1], &body_pos[i], Some(tail_positions));
            } else if i == 0 {
                body_pos[i] = tail_movement(&curr_head, &body_pos[i], None);
            } else {
                body_pos[i] = tail_movement(&body_pos[i - 1], &body_pos[i], None);
            }
        }

        // head = curr_head;
    }

    // print_board(body_pos, 26, &head, start_pos);
    // println!("moved {} spaces", amount);
}

#[allow(dead_code)]
// purely a debug function
fn print_board(
    body_pos: &Vec<(i32, i32)>,
    side_length: i32,
    head_pos: &(i32, i32),
    start_pos: &(i32, i32),
) {
    println!("----------GRID---------");
    println!("head pos: {:?}", head_pos);
    let mut y = side_length;
    while y >= 0 {
        let mut out_line = "".to_string();
        'x_axis: for x in 0..side_length {
            if head_pos.0 == y && head_pos.1 == x {
                out_line += "H";
                continue 'x_axis;
            }

            for (idx, pos) in body_pos.iter().enumerate() {
                if pos.0 == y && pos.1 == x {
                    out_line += &(idx + 1).to_string();
                    continue 'x_axis;
                }
            }

            if start_pos.0 == y && start_pos.1 == x {
                out_line += "s";
                continue 'x_axis;
            }

            out_line += ".";
        }

        println!("{}", out_line);
        y -= 1;
    }
}

#[test]
fn test_tail_movement() {
    let mut unique_fields = UniqueVec::new();

    assert_eq!(
        tail_movement(&(2, 1), &(0, 1), Some(&mut unique_fields)),
        (1, 1)
    );
    assert_eq!(
        tail_movement(&(0, 0), &(2, 0), Some(&mut unique_fields)),
        (1, 0)
    );
    assert_eq!(
        tail_movement(&(0, 0), &(0, 2), Some(&mut unique_fields)),
        (0, 1)
    );
    assert_eq!(
        tail_movement(&(0, 2), &(0, 0), Some(&mut unique_fields)),
        (0, 1)
    );

    // test diagonale
    assert_eq!(
        tail_movement(&(2, 2), &(0, 1), Some(&mut unique_fields)),
        (1, 2)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(0, 3), Some(&mut unique_fields)),
        (1, 2)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(4, 1), Some(&mut unique_fields)),
        (3, 2)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(4, 3), Some(&mut unique_fields)),
        (3, 2)
    );

    assert_eq!(
        tail_movement(&(2, 2), &(1, 4), Some(&mut unique_fields)),
        (2, 3)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(1, 0), Some(&mut unique_fields)),
        (2, 1)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(3, 4), Some(&mut unique_fields)),
        (2, 3)
    );
    assert_eq!(
        tail_movement(&(2, 2), &(3, 0), Some(&mut unique_fields)),
        (2, 1)
    );

    // test non diagonale movement
    assert_eq!(tail_movement(&(2, 2), &(1, 1), None), (1, 1));
    assert_eq!(tail_movement(&(2, 2), &(1, 3), None), (1, 3));
    assert_eq!(tail_movement(&(2, 2), &(3, 1), None), (3, 1));
    assert_eq!(tail_movement(&(2, 2), &(3, 3), None), (3, 3));

    assert_eq!(7, unique_fields.len());
}
