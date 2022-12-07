use std::collections::VecDeque;

use crate::seventh::command::CurrentCommand::{CD, DIR, FILE, LS, NEW};

use self::item::{Dir, FSItem, File};

mod command;
mod item;

pub fn main(input_str: &String) {
    let input_parts: Vec<&str> = input_str
        .split_ascii_whitespace()
        .map(|x| x.trim())
        .collect();

    let mut curr_dir: VecDeque<Dir> = VecDeque::new();
    let mut all_dirs = Vec::new();
    let mut curr_command = NEW;
    let mut curr_size = 0;

    for input in input_parts {
        if input == "$" {
            curr_command = NEW;
            continue;
        }

        match curr_command {
            NEW => {
                match input {
                    "ls" => {
                        curr_command = LS;
                    }
                    "cd" => {
                        curr_command = CD;
                    }
                    _ => {}
                }
                continue;
            }
            CD => match input {
                ".." => {
                    if curr_dir.len() == 1 {
                        continue;
                    }

                    let read_dir = curr_dir.pop_front();

                    match read_dir {
                        Some(dir) => {
                            all_dirs.push(dir.clone());
                            curr_dir.get_mut(0).unwrap().add_item(FSItem::Dir(dir));
                        }
                        None => {}
                    }
                }
                dir => {
                    let dir = Dir::new(dir.to_string());
                    curr_dir.push_front(dir);
                }
            },
            LS => match input {
                "dir" => curr_command = DIR,
                size => {
                    curr_size = size.parse().unwrap();
                    curr_command = FILE;
                }
            },
            DIR => curr_command = LS,
            FILE => {
                curr_dir
                    .get_mut(0)
                    .unwrap()
                    .add_item(FSItem::File(File::new(input.to_string(), curr_size)));
                curr_command = LS;
            }
        }
    }

    while curr_dir.len() > 1 {
        let pop = curr_dir.pop_front();
        curr_dir
            .get_mut(0)
            .unwrap()
            .add_item(FSItem::Dir(pop.unwrap()));
    }

    let needed_size = 30000000 - (70000000 - curr_dir.get(curr_dir.len() - 1).unwrap().size);

    all_dirs.append(&mut curr_dir.into());

    let mut all_under_size = 0;
    let mut curr_delete_size = 0;

    for dir in all_dirs {
        if dir.size <= 100000 {
            all_under_size += dir.size;
        }

        if dir.size >= needed_size && (dir.size < curr_delete_size || curr_delete_size == 0) {
            curr_delete_size = dir.size;
        }
    }

    println!("Size of all folders under 100000: {}", all_under_size);

    println!("Required space to free up: {}", needed_size);

    println!("Smallest folder size to delete: {}", curr_delete_size);
}
