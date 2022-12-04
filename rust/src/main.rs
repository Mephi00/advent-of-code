use std::fs::read_to_string;

mod first;
mod fourth;
mod second;
mod third;

fn main() {
    let mut date: i32;
    let input_str;
    loop {
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

    match date {
        1 => first::main(&input_str),
        2 => second::main(&input_str),
        3 => third::main(&input_str),
        4 => fourth::main(&input_str),
        _ => println!("Not supported yet"),
    };
}
