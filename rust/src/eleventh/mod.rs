use core::cmp::Ordering;
use std::cell::RefCell;

use num_bigint::BigUint;

use crate::eleventh::monkey::Monkey;

mod monkey;

pub fn main(input_str: &String) {
    let input_lines: Vec<&str> = input_str.split("\n").collect();

    let mut monkeys = read_input(input_lines.clone());

    let mut monkeys_second = read_input(input_lines);

    let min_div = monkeys
        .iter()
        .map(|x| x.borrow().test_num)
        .fold(1, |a, b| a * b);

    let min_div_second = monkeys_second
        .iter()
        .map(|x| x.borrow().test_num)
        .fold(1, |a, b| a * b);

    do_turns(&monkeys, 20, true, min_div);

    do_turns(&monkeys_second, 10000, false, min_div_second);

    println!("len of monkey vec: {}", monkeys.len());

    monkeys.sort_by(|a, b| {
        if a.borrow().num_inspected_items > b.borrow().num_inspected_items {
            return Ordering::Less;
        } else if a.borrow().num_inspected_items == b.borrow().num_inspected_items {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    });

    monkeys_second.sort_by(|a, b| {
        if a.borrow().num_inspected_items > b.borrow().num_inspected_items {
            return Ordering::Less;
        } else if a.borrow().num_inspected_items == b.borrow().num_inspected_items {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    });

    println!(
        "monkey busines of top monkey: {}",
        monkeys_second[0].borrow().num_inspected_items
    );

    println!(
        "monkey busines of second monkey: {}",
        monkeys_second[1].borrow().num_inspected_items
    );

    println!(
        "total monkey business of top 2 after 20 iterations: {}",
        monkeys[0].borrow().num_inspected_items * monkeys[1].borrow().num_inspected_items
    );

    println!(
        "total monkey business of top 2 after 1000 iterations: {}",
        monkeys_second[0].borrow().num_inspected_items
            * monkeys_second[1].borrow().num_inspected_items
    );
}

fn do_turns(
    monkeys: &Vec<RefCell<Monkey>>,
    num_iter: u32,
    include_reduction: bool,
    min_divisor: u128,
) {
    for _ in 0..num_iter {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            let test_true_num = monkey.test_true;
            let test_false_num = monkey.test_false;

            let mut test_true = monkeys[test_true_num].borrow_mut();

            if test_true_num == test_false_num {
                if include_reduction {
                    monkey.turn(&mut test_true, None, min_divisor);
                } else {
                    monkey.turn_without_reduction(&mut test_true, None, min_divisor);
                }
            }

            let mut test_false = monkeys[test_false_num].borrow_mut();
            if include_reduction {
                monkey.turn(&mut test_true, Some(&mut test_false), min_divisor);
            } else {
                monkey.turn_without_reduction(&mut test_true, Some(&mut test_false), min_divisor);
            }
        }
    }
}

fn read_input(input_list: Vec<&str>) -> Vec<RefCell<Monkey>> {
    let mut monkeys: Vec<RefCell<Monkey>> = Vec::new();

    let mut test_num = 1;
    let mut test_true = 0;
    let mut test_false = 0;
    let mut starting_items: Vec<BigUint> = Vec::new();
    let mut operation: Box<dyn Fn(&BigUint) -> BigUint> = Box::new(|_| BigUint::default());
    let mut monkey_line_counter = 0;

    for (idx, line) in input_list.iter().enumerate() {
        match monkey_line_counter {
            0 => {
                monkey_line_counter += 1;
                continue;
            }
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

                if function[2] == "old" {
                    match function[1] {
                        "+" => {
                            operation = Box::new(move |input| input.clone() + input);
                        }
                        "-" => {
                            operation = Box::new(move |input| input.clone() - input);
                        }
                        "*" => {
                            operation = Box::new(move |input| input.clone() * input);
                        }
                        "/" => {
                            operation = Box::new(move |input| input.clone() / input);
                        }
                        _ => {}
                    }
                } else {
                    let number: u128 = function[2]
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
        if line.is_empty() || idx == input_list.len() - 1 {
            monkeys.push(RefCell::new(Monkey::new(
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
            operation = Box::new(|_| BigUint::default());
            monkey_line_counter = 0;
        }
    }

    monkeys
}
