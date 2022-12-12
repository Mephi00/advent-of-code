use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    sync::Arc,
};

use crate::eleventh::monkey::Monkey;

mod monkey;

pub fn main(input_str: &String) {
    let input_lines = input_str.split("\n");

    let mut monkeys: Vec<Cell<Monkey>> = Vec::new();

    let mut test_num = 1;
    let mut test_true = 0;
    let mut test_false = 0;
    let mut starting_items: Vec<u32> = Vec::new();
    let mut operation: Box<dyn Fn(u32) -> u32> = Box::new(|_| 0);
    let mut monkey_line_counter = 0;

    for line in input_lines {
        match monkey_line_counter {
            0 => continue,
            1 => {
                starting_items = line
                    .split(":")
                    .nth(1)
                    .expect(&format!("couldnt split {} at :", line))
                    .split(",")
                    .map(|x| x.trim().parse().expect(&format!("couldnt parse {}", x)))
                    .collect()
            }
            2 => {
                let function: Vec<&str> = line
                    .split("=")
                    .nth(1)
                    .expect(&format!("couldnt split {} at =", line))
                    .split_ascii_whitespace()
                    .map(|f| f.trim())
                    .collect();

                let number: u32 = function[2]
                    .parse()
                    .expect(&format!("couldnt parse {}", function[2]));

                match function[1] {
                    "+" => {
                        operation = Box::new(move |input| input + number);
                    }
                    "-" => {
                        operation = Box::new(move |input| input - number);
                    }
                    "*" => {
                        operation = Box::new(move |input| input * number);
                    }
                    "/" => {
                        operation = Box::new(move |input| input / number);
                    }
                    _ => {}
                }
            }
            3 => {
                test_num = line
                    .split("by")
                    .nth(1)
                    .expect(&format!("couldnt split {} at 'by'", line))
                    .trim()
                    .parse()
                    .expect(&format!("couldnt parse {}", line));
            }
            4 => {
                let splites: Vec<&str> = line.split_ascii_whitespace().map(|f| f.trim()).collect();

                test_true = splites[splites.len() - 1]
                    .parse()
                    .expect(&format!("couldnt parse {}", splites[splites.len() - 1]));
            }
            5 => {
                let splites: Vec<&str> = line.split_ascii_whitespace().map(|f| f.trim()).collect();

                test_false = splites[splites.len() - 1]
                    .parse()
                    .expect(&format!("couldnt parse {}", splites[splites.len() - 1]));
            }
            _ => {}
        }

        // increase line count and test if input reset and push monkey to list
        monkey_line_counter += 1;
        if line.is_empty() {
            monkeys.push(Cell::new(Monkey::new(
                starting_items,
                operation,
                test_num,
                test_true,
                test_false,
            )));

            test_num = 1;
            test_false = 0;
            test_true = 0;
            starting_items = Vec::new();
            operation = Box::new(|_| 0);
            monkey_line_counter = 0;
        }
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].get_mut();
            let test_true = monkeys[monkey.test_true].get_mut();
            let test_false = monkeys[monkey.test_false].get_mut();
            monkey.turn(&mut test_true, &mut test_false);
        }
    }
}
