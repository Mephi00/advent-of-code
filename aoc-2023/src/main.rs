use std::fs::read_to_string;

mod eight;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;

fn main() {
    let debug_day = 10;
    let mut date: i32;
    let mut filename;
    let input_str;
    loop {
        if debug_day > 0 {
            println!("Test input?: [y/n]");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            if input.trim().to_lowercase() == "y" {
                filename = "test".to_string();
            } else {
                filename = debug_day.to_string();
            }
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
            filename = date.to_string();
        }

        if date > 25 || date < 1 {
            println!("Input is outside the expected days");
            continue;
        } else {
            let input_str_res = read_to_string(format!("./inputs/input_{}.txt", filename));

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
        1 => one::exec(input_str),
        2 => two::exec(input_str),
        3 => three::exec(input_str),
        4 => four::exec(input_str),
        5 => five::exec(input_str),
        6 => six::exec(input_str),
        7 => seven::exec(input_str),
        8 => eight::exec(input_str),
        9 => nine::exec(input_str),
        10 => ten::exec(input_str),
        _ => println!("Not supported yet"),
    };
}
