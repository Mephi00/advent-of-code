use std::fs::read_to_string;

mod eighth;
mod eleventh;
mod fifth;
mod first;
mod fourth;
mod ninth;
mod second;
mod seventh;
mod sixth;
mod tenth;
mod third;
mod thirteenth;
mod twelth;

fn main() {
    let debug_day = 13;
    let mut date: i32;
    let input_str;
    loop {
        if debug_day > 0 {
            date = debug_day;
        } else {
            println!("Input the day:");
            let mut input = String::new();
            let std_in = std::io::stdin().read_line(&mut input);

            if std_in.is_err() {
                println!("Couldn't read the input");
                continue;
            }

            input = input.trim().to_string();

            date = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Couldn't parse the input to a number");
                    continue;
                }
            };
        }

        if date > 25 || date < 1 {
            println!("Input is outside the expected days");
            continue;
        } else {
            let input_str_res = read_to_string(format!("../inputs/input_{}.txt", date));

            if input_str_res.is_err() {
                println!("Input is not yet supported");
                continue;
            } else {
                input_str = input_str_res.unwrap();
                break;
            }
        }
    }

    println!("-----------------------------results-------------------------------");

    match date {
        1 => first::main(&input_str),
        2 => second::main(&input_str),
        3 => third::main(&input_str),
        4 => fourth::main(&input_str),
        5 => fifth::main(&input_str),
        6 => sixth::main(&input_str),
        7 => seventh::main(&input_str),
        8 => eighth::main(&input_str),
        9 => ninth::main(&input_str),
        10 => tenth::main(&input_str),
        11 => eleventh::main(&input_str),
        12 => twelth::main(&input_str),
        13 => thirteenth::main(&input_str),
        _ => println!("Not supported yet"),
    };
}
