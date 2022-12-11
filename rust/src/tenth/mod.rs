pub fn main(input_str: &String) {
    let mut input_list = input_str.split("\n").filter(|x| !x.is_empty());
    let mut register_x = 1;

    let mut apply = false;
    let mut total_cycles = 0;

    let mut signal_strengths = Vec::new();
    let mut register_change = 0;

    let mut current_row = Vec::new();

    loop {
        total_cycles += 1;

        draw_pixels(register_x, &mut current_row);

        if apply {
            if total_cycles == 20 || (total_cycles - 20) % 40 == 0 {
                signal_strengths.push(total_cycles * register_x);
            }

            register_x += register_change;
            register_change = 0;
            apply = false;

            continue;
        }

        // this might have to be moved due to the definition of "during" in the description of the task
        let input = input_list.next();

        if input.is_none() {
            break;
        }

        let command_input: Vec<&str> = input
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.trim())
            .collect();

        match command_input[0] {
            "noop" => {}
            "addx" => {
                apply = true;
                let change = command_input[1]
                    .parse()
                    .expect(&format!("couldnt parse {}", command_input[1]));

                register_change = change;
            }
            _ => {}
        }

        if total_cycles == 20 || (total_cycles - 20) % 40 == 0 {
            signal_strengths.push(total_cycles * register_x);
        }
    }

    println!("{:?}", signal_strengths);

    println!(
        "sum of signal strengths: {}",
        signal_strengths.iter().sum::<i32>()
    )
}

fn draw_pixels(sprite_center: i32, curr_row: &mut Vec<&str>) {
    if sprite_center <= 0 {
        if curr_row.len() > 0 {
            curr_row.push(".");
        } else if sprite_center >= -1 {
            curr_row.push("#");
        } else {
            curr_row.push(".");
        }
    } else if curr_row.len() == (sprite_center - 1).try_into().unwrap()
        || curr_row.len() == sprite_center.try_into().unwrap()
        || curr_row.len() == (sprite_center + 1).try_into().unwrap()
    {
        curr_row.push("#");
    } else {
        curr_row.push(".")
    }

    if curr_row.len() == 40 {
        println!(
            "{}",
            curr_row.iter().flat_map(|s| s.chars()).collect::<String>()
        );

        curr_row.truncate(0);
    }
}
