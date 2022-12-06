use std::collections::VecDeque;

pub fn main(input_str: &String) {
    println!(
        "Number of characters before the package start: {}",
        num_chars_until_count_unique(4, input_str)
    );

    println!(
        "Number of characters before the message start: {}",
        num_chars_until_count_unique(14, input_str)
    );
}

fn num_chars_until_count_unique(count: usize, input_str: &String) -> usize {
    let mut char_buf = VecDeque::new();

    let mut char_counter = 0;

    for (index, char) in input_str.chars().enumerate() {
        if char_buf.len() == count {
            char_buf.pop_front();
        }

        char_buf.push_back(char);

        if only_unique(&char_buf) {
            if char_buf.len() == count {
                char_counter = index + 1;
                break;
            }
        }
    }

    char_counter
}

fn only_unique(char_buf: &VecDeque<char>) -> bool {
    for (index, char) in char_buf.iter().enumerate() {
        let mut comp_buf = char_buf.clone();
        comp_buf.remove(index);
        if comp_buf.contains(char) {
            return false;
        }
    }

    true
}
